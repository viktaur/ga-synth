use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::subtractive::{SubtractiveIndividual};

const TARGET: &str = "audio_samples/440hz_sine.wav";

const GENERATIONS: u32 = 500;

fn main() {
    (3..10).into_par_iter().for_each(|i: u32| run(2u32.pow(i)));
    println!("All simulations completed.");
}

fn run(population: u32) {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    println!("Running simulation with population {}", population);

    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator.clone())
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(population)
        .n_random_additions(0)
        .mutation_rate(0.05)
        .max_generations(GENERATIONS)
        .signal_export(&format!("test_5/{}.wav", population))
        .csv_export(&format!("test_5/{}.csv", population))
        .build();

    simulation.run().expect("Simulation should have completed.");
}