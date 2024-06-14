use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::subtractive::{SubtractiveIndividual};

const TARGET: &str = "audio_samples/440hz_sine.wav";

const GENERATIONS: u32 = 500;

const POPULATION: u32 = 100;

fn main() {
    (0..10).into_par_iter().for_each(|i| run(0.05 * i as f32));
    println!("All simulations completed.");
}

fn run(mutation_rate: f32) {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();
    
    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator.clone())
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        .n_random_additions(4)
        .mutation_rate(mutation_rate)
        .max_generations(GENERATIONS)
        .signal_export(&format!("test_6/{}.wav", mutation_rate))
        .csv_export(&format!("test_6/{}.csv", mutation_rate))
        .build();

    simulation.run().expect("Simulation should have completed.");
}