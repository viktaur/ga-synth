# ga-synth
This library enables sound optimisation through the use of [genetic algorithms (GAs)](https://en.wikipedia.org/wiki/Genetic_algorithm). Given a target sound wave, the GA searches for an optimal set of parameters that approximates the produced signal. The GA configuration, fitness evaluation method used, and sound synthesis components used are all factors that affect the quality of the solution.

The core of the GA is defined [here](src/simulation/algorithms/genetic.rs).

For a complete overview of the project, please read [this](paper.pdf).
