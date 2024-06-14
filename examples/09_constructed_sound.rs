use std::sync::Arc;
use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::signal_processing::Signal;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::components::oscillator::OscillatorComponent;
use ga_synth::simulation::synthesis_methods::additive::AdditiveIndividual;
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const TARGET: &str = "audio_samples/custom.wav";

const POPULATION: u32 = 1000;
const GENERATIONS: u32 = 500;
const N_SIMS: u8 = 10;

fn main() {
    // construct_sound()
    // subtractive_multiple();
    // additive_multiple();
    subtractive();
}

fn construct_sound() {
    let mut signal = Signal::default();

    let oscillator = OscillatorComponent {
        freq: 520.0,
        sine_amp: 0.3,
        sine_phase: 0.2,
        square_amp: 0.3,
        square_phase: 0.1,
        saw_amp: 0.4,
        saw_phase: 0.0,
    };

    signal.apply_oscillator(oscillator);
    signal.to_wav("custom.wav").unwrap()
}

fn subtractive() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator.clone())
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        .n_random_additions(5)
        .mutation_rate(0.15)
        .max_generations(GENERATIONS)
        .signal_export(&format!("test_9/out.wav"))
        .csv_export(&format!("test_9/out.csv"))
        .build();

    simulation.run().expect("Simulation should have completed.");
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
            .signal_export(&format!("test_9/a/{}.wav", i))
            .csv_export(&format!("test_9/a/{}.csv", i))
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
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_9/b/{}.wav", i))
            .csv_export(&format!("test_9/b/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });

    println!("All additive simulations completed");
}