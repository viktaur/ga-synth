use rayon::prelude::*;
use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::algorithms::hillclimbing::{HillClimberBuilder, HillClimbingSimulation};
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const TARGET: &str = "audio_samples/440hz_sine.wav";

const POPULATION: u32 = 100;
const GENERATIONS: u32 = 500;
const N_SIMS: u8 = 10;

fn main() {
    
    ga_multiple();
    hillclimber_multiple()
}

fn ga_multiple() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running GA simulation {i}/{N_SIMS}");

        let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
            .generator(generator.clone())
            .population_evolution(PopulationEvolution::Constant)
            .initial_population(POPULATION)
            .n_random_additions(4)
            .mutation_rate(0.05)
            .max_generations(GENERATIONS)
            .signal_export(&format!("test_2/a/{}.wav", i))
            .csv_export(&format!("test_2/a/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });
}

fn hillclimber_multiple() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file(TARGET)
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    (0..N_SIMS).into_par_iter().for_each(|i| {
        println!("Running hill climbing simulation {i}/{N_SIMS}");
        
        let mut simulation: HillClimbingSimulation<SubtractiveIndividual> = HillClimberBuilder::new()
            .generator(generator.clone())
            .max_iterations((POPULATION / 2) * GENERATIONS)
            .max_unsuccessful_iters(10000)
            .signal_export(&format!("test_2/b/{}.wav", i))
            .csv_export(&format!("test_2/b/{}.csv", i))
            .build();

        simulation.run().expect("Simulation should have completed.");
    });
}