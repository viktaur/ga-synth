use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const TARGET: &str = "audio_samples/440hz_sine.wav";

const POPULATION: u32 = 100;
const GENERATIONS: u32 = 500;
const N_SIMS: u8 = 10;

fn main() {
    fitness_mse();
    fitness_time_domain();
}

fn fitness_mse() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA MSE fitness simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_3/a/{}.wav", i))
            .csv_export(&format!("test_3/a/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All MSE fitness simulations completed.")
}

fn fitness_time_domain() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::TimeDomainEuclidean)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA Time-Domain fitness simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_3/b/{}.wav", i))
            .csv_export(&format!("test_3/b/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All Time-Domain Fitness simulations completed.")
}