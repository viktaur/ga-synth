use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::additive::AdditiveIndividual;
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const POPULATION: u32 = 150;
const GENERATIONS: u32 = 1000;
const N_SIMS: u8 = 10;

fn main() {
    subtractive("audio_samples/sawtooth440.wav");
    additive("audio_samples/sawtooth440.wav");
    
    subtractive("audio_samples/synth_shimmer.wav");
    additive("audio_samples/synth_shimmer.wav");
}

fn subtractive(target: &str) {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(target)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA subtractive simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.1)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_7/{}/a/{}.wav", target, i))
            .csv_export(&format!("test_7/{}/a/{}.csv", target, i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All subtractive simulations completed");
}

fn additive(target: &str) {
    let generator = AdditiveIndividual::new_generator()
        .target_file(target)
        .fitness_type(FitnessType::FreqDomainMSE)
        .harmonics();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA subtractive simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<AdditiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_7/{}/b/{}.wav", target, i))
            .csv_export(&format!("test_7/{}/b/{}.csv", target, i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All additive simulations completed");
}