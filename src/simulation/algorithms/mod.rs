/// Genetic algorithms are a global optimisation technique inspired by biological mechanisms like
/// evolution.
pub mod genetic;

/// A hillclimber is one of the simplest stochastic optimisation techniques that works by exploring
/// the best nearest neighbour. It is not as effective at finding a global optima as a genetic
/// algorithm, but can be useful as a performance reference.
pub mod hillclimbing;