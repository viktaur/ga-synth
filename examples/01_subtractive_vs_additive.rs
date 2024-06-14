use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::additive::AdditiveIndividual;
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;
use rayon::prelude::*;

const TARGET: &str = "audio_samples/440hz_sine.wav";

const POPULATION: u32 = 100;
const GENERATIONS: u32 = 500;
const N_SIMS: u8 = 10;

fn main() {
    // subtractive_multiple();
    // additive_multiple();
    subtractive()
}

fn subtractive_multiple() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA subtractive simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_1/a/{}.wav", i))
            .csv_export(&format!("test_1/a/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All subtractive simulations completed");
}

fn additive_multiple() {
    let generator = AdditiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .harmonics();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running a GA additive simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<AdditiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.1)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_1/b/{}.wav", i))
            .csv_export(&format!("test_1/b/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All additive simulations completed");
}

fn subtractive() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file("audio_samples/440hz_sine.wav")
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator)
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        .n_random_additions(4)
        .mutation_rate(0.05)
        .max_generations(GENERATIONS)
        .signal_export("test_1_a.wav")
        .csv_export("test_1_a.csv")
        .build();

    simulation.run().expect("Simulation should have completed.");
}

fn additive() {
    let generator = AdditiveIndividual::new_generator()
        .target_file("audio_samples/440hz_sine.wav")
        .fitness_type(FitnessType::FreqDomainMSE)
        .harmonics();
    
    let mut simulation: GASimulation<AdditiveIndividual> = GASimulationBuilder::new()
        .generator(generator)
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        .n_random_additions(4)
        .mutation_rate(0.1)
        .max_generations(GENERATIONS)
        .signal_export("test_1_b.wav")
        .csv_export("test_1_b.csv")
        .build();
    
    simulation.run().expect("Simulation should have completed.");
}