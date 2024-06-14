use ga_synth::FitnessType;
use ga_synth::simulation::algorithms::genetic::{GASimulation, GASimulationBuilder, Individual, IndividualGenerator, PopulationEvolution};
use ga_synth::simulation::synthesis_methods::additive::AdditiveIndividual;
use ga_synth::simulation::synthesis_methods::subtractive::SubtractiveIndividual;

const POPULATION: u32 = 150;
const GENERATIONS: u32 = 300;

fn main() {
    let generator = SubtractiveIndividual::new_generator()
        .target_file("audio_samples/440hz_sine.wav")
        .fitness_type(FitnessType::FreqDomainMSE)
        .oscillator();

    let mut simulation: GASimulation<SubtractiveIndividual> = GASimulationBuilder::new()
        .generator(generator)
        .population_evolution(PopulationEvolution::Constant)
        .initial_population(POPULATION)
        // .n_random_additions(4)
        .mutation_rate(0.05)
        .max_generations(GENERATIONS)
        .signal_export("viva/test.wav")
        .csv_export("viva/test.csv")
        .build();

    simulation.run().expect("Simulation should have completed.");
}