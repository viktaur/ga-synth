use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use spectrum_analyzer::error::SpectrumAnalyzerError;

/// Errors that can be encountered during the execution of the genetic algorithm.
// TODO make them more specific, explaining the reason why something went wrong.
#[derive(Debug)]
pub enum GeneticSimulationError {
    OffspringNotProduced,
    RandomIndividualNotGenerated
}

impl Error for GeneticSimulationError {}

impl fmt::Display for GeneticSimulationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO include self in output
        write!(f, "Something went wrong")
    }
}

#[derive(Debug)]
pub enum HillClimbingSimulationError {
    NoFitterNeighbourFound,
    GeneratorMissing,
    TargetMissing,
}

impl Error for HillClimbingSimulationError {}

impl fmt::Display for HillClimbingSimulationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO include self in output
        write!(f, "Something went wrong")
    }
}

/// Errors that can be encountered during the signal processing, including synthesis and comparison.
#[derive(Debug)]
pub enum SignalProcessingError {
    InvalidSpectrum(SpectrumAnalyzerError),
    CouldNotReadFromFile(&'static str),
}

impl Error for SignalProcessingError {}

impl fmt::Display for SignalProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO include self in output
        write!(f, "Something went wrong")
    }
}