use std::borrow::Borrow;
use crate::signal_processing::Signal;
use crate::utils::sigmoid;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::fmt::{Binary, Debug};
use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use itertools::{Itertools};
use crate::error::{GeneticSimulationError};
use rayon::prelude::*;
use crate::{FitnessType};
use crate::analytics::{GenerationRow, Recorder};
use anyhow::Result;

/// Represents a simulation of the genetic algorithm for a generic sound signal_processing method.
#[derive(Clone, Debug)]
pub struct GASimulation<T: Individual> {
    /// Current generation number.
    pub generation: u32,
    /// The probability of seeing a mutation in a specific gene.
    pub mutation_rate: f32,
    /// The number of generations the simulation will run for.
    pub max_generations: u32,
    /// The population of the current generation sorted by fitness.
    pub population: Vec<T>,
    /// The signal we are using as target and upon which the fitness function is defined.
    pub target: Signal,
    /// Number of randomly added individuals on each generation.
    pub n_random_additions: u32,
    /// The size of the population at the beginning of the simulation.
    pub initial_population: u32,
    /// How the population evolves as new individuals are considered.
    pub population_evolution: PopulationEvolution,
    /// Number of individuals produced in a generation.
    pub offspring: u32,
    /// Fundamental frequency of the fittest individual.
    pub fundamental: Option<f32>,
    /// Generator used to bring new randomised individuals.
    pub generator: T::Generator,
    /// Whether the simulation should be exported to a CSV file and what file name.
    pub csv_export: Option<String>,
    /// Whether the fittest individual should be exported to a WAV file and what file name.
    pub signal_export: Option<String>,
}

pub struct GASimulationBuilder<T: Individual> {
    pub generator: Option<T::Generator>,
    pub target: Option<Arc<Signal>>,
    pub initial_population: u32,
    pub n_random_additions: u32,
    pub mutation_rate: f32,
    pub max_generations: u32,
    pub population_evolution: PopulationEvolution,
    pub csv_export: Option<String>,
    pub signal_export: Option<String>
}

impl<T: Individual> Default for GASimulationBuilder<T> {
    fn default() -> Self {
        Self {
            generator: None,
            target: None,
            initial_population: 100,
            n_random_additions: 5,
            mutation_rate: 0.05,
            max_generations: 1_000,
            population_evolution: PopulationEvolution::default(),
            csv_export: None,
            signal_export: None,
        }
    }
}

impl<T: Individual> GASimulationBuilder<T> {
    
    /// Creates a new GA simulation builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds the GA simulation builder.
    pub fn build(self) -> GASimulation<T> {
        let generator = self.generator.expect("Expected a generator.");
        let population = GASimulation::init_population(self.initial_population, &generator);
        let target_arc = self.target
            .expect("Expected a reference counter to the target signal.");
        let target = Signal::clone(&*target_arc);

        GASimulation {
            population,
            target,
            generator,
            offspring: 0,
            generation: 0,
            fundamental: None,
            mutation_rate: self.mutation_rate,
            max_generations: self.max_generations,
            n_random_additions: self.n_random_additions,
            initial_population: self.initial_population,
            population_evolution: self.population_evolution,
            csv_export: self.csv_export,
            signal_export: self.signal_export,
        }
    }

    /// Specifies target signal.
    pub fn target(mut self, target: Signal) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Takes an individual generator.
    pub fn generator(mut self, generator: T::Generator) -> Self {
        self.target = Some(generator.get_target());
        self.generator = Some(generator);
        self
    }

    /// Specifies the initial population.
    pub fn initial_population(mut self, initial_population: u32) -> Self {
        self.initial_population = initial_population;
        self
    }

    /// Specifies the number of randomly generated individuals incorporated per generation.
    pub fn n_random_additions(mut self, n_random_additions: u32) -> Self {
        self.n_random_additions = n_random_additions;
        self
    }

    /// Specifies the mutation rate of the simulation.
    pub fn mutation_rate(mut self, mutation_rate: f32) -> Self {
        self.mutation_rate = mutation_rate;
        self
    }

    /// Specifies the number of generations the simulation will run for.
    pub fn max_generations(mut self, max_generations: u32) -> Self {
        self.max_generations = max_generations;
        self
    }

    /// Specifies how the population will evolve over time.
    pub fn population_evolution(mut self, population_evolution: PopulationEvolution) -> Self {
        self.population_evolution = population_evolution;
        self
    }

    /// Takes a CSV file name where the simulation will be exported.
    pub fn csv_export(mut self, file_name: &str) -> Self {
        self.csv_export = Some(file_name.to_string());
        self
    }

    /// Takes a WAV file name where the returned signal will be exported.
    pub fn signal_export(mut self, file_name: &str) -> Self {
        self.signal_export = Some(file_name.to_string());
        self
    }
}

#[derive(Clone, Debug)]
pub enum PopulationEvolution {
    Constant,
    Increasing
}

impl Default for PopulationEvolution {
    fn default() -> Self {
        Self::Constant
    }
}

impl<T: Individual> GASimulation<T> {
    fn init_population(n: u32, generator: &T::Generator) -> Vec<T> {
        let mut vec: Vec<T> = (0..n).into_par_iter().map(|_| generator.generate()).collect();
        vec.par_sort_by(|a, b| b.cmp(a));
        vec
    }

    /// A step in the iteration of the algorithm. Given the current state of the simulation, calculates the next
    /// generation.
    fn next(&mut self) -> Result<(), GeneticSimulationError> {
        // Add n randomly generated individuals to the current population and sort it.
        let mut current_population = self.population.clone();
        let mut random_additions = vec![];
        for _ in 0..self.n_random_additions {
            random_additions.push(self.generator.generate());
        }
        current_population.extend(random_additions);
        current_population.sort_by(|a, b| b.cmp(a));

        // number of selected individuals for the next generation
        let n_selected = match self.population_evolution {
            PopulationEvolution::Constant =>  { self.initial_population as usize / 2 }
            PopulationEvolution::Increasing => { current_population.len() / 2 }
        };

        // construct a new population vec from the n selected individuals
        let mut new_population: Vec<T> = Vec::from(&current_population[0..n_selected]);
        let offspring_mutex: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(vec![]));

        let mut rng = thread_rng();

        for _ in 0..2 {
            new_population.shuffle(&mut rng);
            new_population.par_iter().chunks(2).for_each(|p| {
                if p.len() == 2 {
                    if let Some(c) = p[0].crossover(p[1], self.mutation_rate) {
                        let mut guard = offspring_mutex.lock().unwrap();
                        guard.push(c);
                    }
                }
            });
        }

        let offspring = offspring_mutex.lock().unwrap();
        
        // update offspring for stats purposes
        self.offspring = offspring.len() as u32;

        // join the new population and offspring vecs, then sort it
        new_population.extend(offspring.to_vec());
        new_population.sort_by(|a, b| b.cmp(a));
        
        // update generation population with the new one
        self.population = new_population;

        // update fundamental frequency and print current population
        let fittest: &T = self.population.first().expect("There should be a fittest individual in the population");
        self.fundamental = fittest.get_fundamental();
        
        if self.generation % 10 == 0 {
            println!("Gen: {}, - {:?}", self.generation, fittest.dbg());
        }
        
        // increase generation count
        self.generation += 1;
        
        Ok(())
    }


    /// Runs a genetic algorithm simulation.
    pub fn run(&mut self) -> Result<T, GeneticSimulationError> {
        // let mut generation = 0;
        let mut recorder: Recorder<GenerationRow> = Recorder::new();

        if self.csv_export.is_some() {
            recorder.add_record(self.into());
        }

        while self.generation < self.max_generations {
            // calculate the next generation and update state
            self.next()?;
            
            // update the record
            if self.csv_export.is_some() {
                recorder.add_record(self.into());
            }
        }

        if let Some(file_name) = &self.csv_export {
            recorder.to_csv(file_name).expect("Exporting to CSV should have been successful");
        }

        // Once the iteration is finished, we select the fittest in the final population
        let fittest: T = self.population.first()
            .expect("There should be a fittest individual in the population.").to_owned();
        println!("{:?}", fittest.dbg());

        if let Some(file_name) = &self.signal_export {
           fittest.to_signal().to_wav(file_name)
               .expect("Exporting to a WAV file should have been successful.")
        }

        Ok(fittest)
    }
}

/// Template for generating an individual with a certain configuration. The implementations for
/// each generator provide a way to specify the components present in a synthesis method.
pub trait IndividualGenerator<T: Individual>: Sized {
    /// Creates a new individual generator.
    fn new() -> Self;

    /// Generates an Individual having specified the components present.
    fn generate(&self) -> T;

    /// Specifies a target signal.
    fn target(self, target: Arc<Signal>) -> Self;
    
    /// Specifies the target sound by taking the URI of the file containing it.
    fn target_file(self, file_path: &str) -> Self {
        let file_in = File::open(file_path)
            .expect("Expected a target file in the specified directory.");
        let target = Signal::from_wav_file(file_in)
            .expect("Target file should have been converted into signal.");
        self.target(Arc::new(target))
    }

    /// Specifies the fitness evaluation method to be used.
    fn fitness_type(self, fitness_type: FitnessType) -> Self;
    
    /// Retrieves the target signal from the generator.
    fn get_target(&self) -> Arc<Signal>;
}

pub trait Individual: Clone + Ord + Debug + Send + Sync {
    type Generator: IndividualGenerator<Self> + Sync;

    fn new_generator() -> Self::Generator;

    /// Returns a clone of the `Rc<Signal>` object holding the target signal.
    fn get_target(&self) -> Arc<Signal>;

    /// Getter method used to return the `fitness` field from the implementations.
    // fn get_fitness(&self) -> Option<f32>;

    /// Defines how 'fit' the individual is, i.e. how close is the individual to the target
    /// sound wave, by comparing it to the frequency spectrum.
    fn fitness(&self) -> f32;

    fn get_fitness_type(&self) -> FitnessType;

    fn calculate_fitness(&self) -> f32 {
        match self.get_fitness_type() {
            FitnessType::FreqDomainMSE => self.freq_domain_mse_fitness(),
            FitnessType::TimeDomainEuclidean => self.time_domain_euclidean_fitness(),
            // FitnessType::TimeDomainCrossCorr => self.time_domain_cross_corr_fitness()
        }
    }

    fn freq_domain_mse_fitness(&self) -> f32 {
        let mse = self.to_signal().freq_spectrum_mse(&self.get_target()).expect("MSE should be valid");
        let cost = (mse / 1000.0).log10().exp();

        // the higher the total cost, the lower the fitness
        2.0 * sigmoid(-cost)
    }

    fn time_domain_euclidean_fitness(&self) -> f32 {
        let distance= self.to_signal().euclidean_distance(&self.get_target());
        let cost = (distance / 500.0).log10().exp();

        // the higher the total cost, the lower the fitness
        2.0 * sigmoid(-cost)
    }

    fn time_domain_cross_corr_fitness(&self) -> f32 {
        todo!()
    }

    /// Replaces the fitness field with the calculated fitness value
    fn include_fitness(self) -> Self;

    /// Returns an offspring from two individuals. r specifies the mutation rate represented as the likelihood
    /// for each gene to mutate
    fn crossover(&self, other: &Self, r: f32) -> Option<Self>
    where
        Self: Sized;

    fn to_signal(&self) -> Signal;

    fn evolve(&self, step_size: f32) -> Self;

    // fn generate_neighbour(&self, step_size: f32) -> Self;

    fn dbg(&self) -> String;
    
    fn get_fundamental(&self) -> Option<f32>;
}



#[cfg(test)]
mod tests {
    use crate::simulation::synthesis_methods::subtractive::SubtractiveIndividual;
    use super::*;

    #[test]
    fn test_increasing_population_even() {
        let target = Signal::default();
        let generator = SubtractiveIndividual::new_generator()
            .target(Arc::new(target.clone()))
            .oscillator();

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .initial_population(100)
            .n_random_additions(4)
            .population_evolution(PopulationEvolution::Increasing)
            .target(Signal::default())
            .generator(generator)
            .build();

        assert_eq!(simulation.population.len(), 100);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 104);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 108);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 112);
    }

    #[test]
    fn test_increasing_population_odd() {
        let target = Signal::default();
        let generator = SubtractiveIndividual::new_generator()
            .target(Arc::new(target.clone()))
            .oscillator();

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .initial_population(100)
            .n_random_additions(3)
            .population_evolution(PopulationEvolution::Increasing)
            .target(Signal::default())
            .generator(generator)
            .build();

        // population should grow by floor(n)
        assert_eq!(simulation.population.len(), 100);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 101);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 104);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 105);
    }

    #[test]
    fn test_constant_population() {
        let target = Signal::default();
        let generator = SubtractiveIndividual::new_generator()
            .target(Arc::new(target.clone()))
            .oscillator();

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .initial_population(100)
            .n_random_additions(4)
            .population_evolution(PopulationEvolution::Constant)
            .target(Signal::default())
            .generator(generator)
            .build();

        assert_eq!(simulation.population.len(), 100);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 100);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 100);
        simulation.next().unwrap();
        assert_eq!(simulation.population.len(), 100);
    }
}
