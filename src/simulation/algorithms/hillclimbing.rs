use std::rc::Rc;
use std::sync::Arc;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::error::HillClimbingSimulationError;
use crate::simulation::algorithms::genetic::{Individual, IndividualGenerator};
use crate::signal_processing::Signal;
use crate::{FitnessType};
use crate::analytics::{IterationRow, Recorder};

pub struct HillClimbingSimulation<T: Individual> {
    /// Fittest individual discovered.
    pub current_individual: T,
    /// Number of individuals generated so far (including rejected ones). 
    pub iteration: u32,
    /// Signal used as target upon which the fitness function is defined.
    pub target: Signal,
    /// Step size at the start of the program.
    pub init_step_size: f32,
    /// Maximum number of iterations the simulation will run for.
    pub max_iterations: u32,
    /// The minimum step size tolerated. If the step size is lower than this value, the program
    /// will terminate.
    pub min_step_size: f32,
    /// Maximum number of unsuccessful interations the simulation will tolerate.
    pub max_unsuccessful_iters: u32,
    /// Fundamental frequency of the current individual.
    pub fundamental: Option<f32>,
    /// Whether the simulation should be exported to a CSV file and what file name.
    pub csv_export: Option<String>,
    /// Whether the fittest individual shoudl be exported ot a WAV file and what file name.
    pub signal_export: Option<String>
}

pub struct HillClimberBuilder<T: Individual> {
    pub generator: Option<T::Generator>,
    pub target: Option<Arc<Signal>>,
    pub init_step_size: f32,
    pub max_iterations: u32,
    pub min_step_size: f32,
    pub max_unsuccessful_iters: u32,
    pub csv_export: Option<String>,
    pub signal_export: Option<String>,
}

// impl<T: Individual> Simulation for HillClimbingSimulation<T> {
//     fn get_fitness_type(&self) -> FitnessType {
//         todo!()
//     }
// }

impl<T: Individual> Default for HillClimberBuilder<T> {
    fn default() -> Self {
        Self {
            generator: None,
            target: None,
            init_step_size: 1.0,
            max_iterations: 3000,
            min_step_size: 0.0001,
            max_unsuccessful_iters: 5000,
            csv_export: None,
            signal_export: None,
        }
    }
}

impl<T: Individual> HillClimberBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> HillClimbingSimulation<T> {
        let generator = self.generator.expect("Generator expected");
        let current_individual = generator.generate();
        let target_rc = self.target
            .expect("Expected a reference counter to the target signal.");
        let target = Signal::clone(&*target_rc);

        HillClimbingSimulation {
            current_individual,
            target,
            iteration: 0,
            init_step_size: self.init_step_size,
            max_iterations: self.max_iterations,
            min_step_size: self.min_step_size,
            max_unsuccessful_iters: self.max_unsuccessful_iters,
            fundamental: None,
            csv_export: self.csv_export,
            signal_export: self.signal_export,
        }
    }

    /// Takes an individual generator than specifies the component layout.
    pub fn generator(mut self, generator: T::Generator) -> Self {
        self.target = Some(generator.get_target());
        self.generator = Some(generator);
        self
    }
    
    /// Takes the file name to which the returned signal will be exported.
    pub fn signal_export(mut self, file_name: &str) -> Self {
        self.signal_export = Some(file_name.into());
        self
    }
    
    /// Takes the CSV file name to which the simulation will be exported.
    pub fn csv_export(mut self, file_name: &str) -> Self {
        self.csv_export = Some(file_name.into());
        self
    }

    /// Specifies the target signal.
    pub fn target(mut self, target: Signal) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Specifies the initial step size of the hill-climbing algorithm.
    pub fn init_step_size(mut self, init_step_size: f32) -> Self {
        self.init_step_size = init_step_size;
        self
    }

    /// Specifies the maximum number of generations the simulation will run for.
    pub fn max_iterations(mut self, max_iterations: u32) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Specifies the minimum step size. If the step size ever goes below this value, the simulation
    /// will terminate.
    pub fn min_step_size(mut self, min_step_size: f32) -> Self {
        self.min_step_size = min_step_size;
        self
    }

    /// The maximum number of consecutive unsuccessful iterations before the simulation is terminated.
    pub fn max_unsuccessful_iters(mut self, max_unsuccessful_iters: u32) -> Self {
        self.max_unsuccessful_iters = max_unsuccessful_iters;
        self
    }
}


impl<T: Individual> HillClimbingSimulation<T> {
    pub fn run(&mut self) -> Result<T, HillClimbingSimulationError> {
        let mut recorder: Recorder<IterationRow> = Recorder::new();
        let mut step_size = self.init_step_size;
        let mut unsuccessful_iters = 0;

        while self.iteration < self.max_iterations {

            if step_size < self.min_step_size {
                println!("Step size too small ({} < {}). Terminating", step_size, self.min_step_size);
                break;
            }

            if unsuccessful_iters >= self.max_unsuccessful_iters {
                println!("{} unsuccessful iterations reached. Terminating", unsuccessful_iters);
                break;
            }
            
            // update the record with current state
            if self.csv_export.is_some() {
                recorder.add_record(self.into());
            }

            println!("Iteration: {}: {}", self.iteration, self.current_individual.dbg());

            let candidate = self.current_individual.evolve(step_size);

            if candidate.fitness() > self.current_individual.fitness() {
                // reduce the step size
                step_size /= 0.95;
                println!("Step size now {step_size}");

                // reset unsuccessful iters
                unsuccessful_iters = 0;
                
                // update the current individual
                self.current_individual = candidate;
                self.fundamental = self.current_individual.get_fundamental();
                println!("Current candidate's fitness is {} and params {:?}",
                         self.current_individual.fitness(),
                         self.current_individual.dbg()
                );
            } else {
                unsuccessful_iters += 1;
            }
            self.iteration += 1;
        }

        println!("{:?}", self.current_individual.dbg());
        
        if let Some(file_name) = &self.csv_export {
            recorder.to_csv(file_name).expect("Exporting to CSV should have been successful.");
        }
        
        if let Some(file_name) = &self.signal_export {
            self.current_individual.to_signal().to_wav(file_name)
                .expect("Exporting to a WAV file should have been successful.")
        }

        Ok(self.current_individual.clone())
    }
}

pub fn evolve_value(val: f32, min_v: f32, max_v: f32, step_size: f32, rng: &mut ThreadRng) -> f32 {
    let dist = (max_v - min_v) * step_size / 2.0;
    rng.gen_range(f32::max(min_v, val-dist)..f32::min(max_v, val+dist))
}
