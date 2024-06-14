use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const TARGET: &str = "audio_samples/440hz_sine.wav";

const GENERATIONS: u32 = 500;

const POPULATION: u32 = 100;

fn main() {
    (0..8).into_par_iter().for_each(|i| run(5 * i));
    println!("All simulations completed.");
}

fn run(n: u32) {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator.clone())
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        .n_random_additions(n)
        .mutation_rate(0.1)
        .max_generations(GENERATIONS)
        .signal_export(&format!("test_8/{}.wav", n))
        .csv_export(&format!("test_8/{}.csv", n))
        .build();

    simulation.run().expect("Simulation should have completed.");
}