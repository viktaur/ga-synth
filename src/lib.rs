/// Everything regarding the sound synthesis and analysis process, including oscillators, operators
/// on amplitude and phase, read and write from/to a WAV file, frequency spectrum analysis,
/// mean squared error (MSE) between two sets of samples, and others. The former two are used to calculate the fitness
/// of an individual with respect to the target given its signal.
pub mod signal_processing;

/// Consists of a miscellaneous group of useful functions.
pub mod utils;

/// Contains the necessary logic to export the simulation data to a CSV file.
pub mod analytics;

/// Everything needed to run a parameter exploration simulation, including the genetic and
/// hillclimber algorithms, different synthesis components and methods and their encoding as individuals.
pub mod simulation;

mod error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FitnessType {
    FreqDomainMSE,
    TimeDomainEuclidean,
    // TimeDomainCrossCorr,
}

impl Default for FitnessType {
    fn default() -> Self {
        FitnessType::FreqDomainMSE
    }
}

//
// pub trait Simulation: Clone + Debug {
//     fn get_fitness_type(&self) -> FitnessType;
// }