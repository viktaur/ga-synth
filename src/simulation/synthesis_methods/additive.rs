use std::cmp::Ordering;
use std::f32::consts::PI;
use std::sync::Arc;
use crate::FitnessType;
use crate::simulation::components::harmonics::HarmonicsComponent;
use crate::signal_processing::{Signal, LENGTH, SAMPLE_RATE};
use crate::simulation::algorithms::genetic::{Individual, IndividualGenerator};

#[derive(Clone, Debug, PartialEq)]
pub struct AdditiveIndividual {
    target: Arc<Signal>,
    fitness_type: FitnessType,
    fitness: Option<f32>,
    harmonics: Option<HarmonicsComponent>
}

#[derive(Clone)]
pub struct AdditiveIndividualGenerator {
    target: Option<Arc<Signal>>,
    fitness_type: FitnessType,
    harmonics: bool
}

impl Eq for AdditiveIndividual {}

impl PartialOrd<Self> for AdditiveIndividual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AdditiveIndividual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness().partial_cmp(&other.fitness()).expect("No fitness value should be NaN")
    }
}

impl AdditiveIndividual {
    fn harmonics_are_valid(&self) -> bool {
        match self.harmonics.as_ref() {
            Some(harmonics) => {
                let fund = harmonics.freq;
                let niquist_freq = SAMPLE_RATE as f32 / 2f32;
                // Ensure all the frequencies are below the Niquist frequency
                (1..=harmonics.amplitudes.len())
                    .all(|i| (fund * i as f32) < niquist_freq)
            },
            _ => true // This doesn't apply if there's no harmonics component.
        }
    }
}

impl Individual for AdditiveIndividual {
    type Generator = AdditiveIndividualGenerator;

    fn new_generator() -> Self::Generator {
        Self::Generator::new()
    }

    fn get_target(&self) -> Arc<Signal> {
        Arc::clone(&self.target)
    }

    fn fitness(&self) -> f32 {
        self.fitness.unwrap_or_else(|| {
            if self.harmonics_are_valid() {
                self.calculate_fitness()
            } else {
                0.0 // Invalidate the individual if the harmonics are not valid
            }
        })
    }

    fn get_fitness_type(&self) -> FitnessType {
        self.fitness_type
    }

    fn include_fitness(mut self) -> Self {
        if self.harmonics_are_valid() {
            self.fitness = Some(self.calculate_fitness())
        } else {
            self.fitness = Some(0.0);
        }
        
        self
    }

    fn crossover(&self, other: &Self, r: f32) -> Option<Self> {
        let harmonics = match (&self.harmonics, &other.harmonics) {
            (Some(s), Some(o)) => s.combine(o, r),
            _ => None
        };

        Some(
            Self {
                target: self.get_target(),
                fitness: None,
                fitness_type: self.fitness_type,
                harmonics
            }.include_fitness()
        )
    }

    fn to_signal(&self) -> Signal {
        let mut signal = Signal::init(LENGTH, SAMPLE_RATE as f32);

        if let Some(harmonics) = &self.harmonics {
            signal.apply_harmonics(harmonics);
        }

        signal
    }

    fn evolve(&self, step_size: f32) -> Self {
        Self {
            target: Arc::clone(&self.target),
            fitness: None,
            fitness_type: self.fitness_type,
            harmonics: self.harmonics.as_ref().map(|har| har.evolve(step_size)),
        }.include_fitness()
    }

    fn dbg(&self) -> String {
        format!("FITNESS: {:?}, Harmonics: {:?}", self.fitness.unwrap_or(0.0), self.harmonics)
    }

    fn get_fundamental(&self) -> Option<f32> {
        Some(self.harmonics.as_ref()?.freq)
    }
}

impl IndividualGenerator<AdditiveIndividual> for AdditiveIndividualGenerator {
    fn new() -> Self {
        AdditiveIndividualGenerator {
            target: None,
            fitness_type: FitnessType::default(),
            harmonics: false
        }
    }

    fn generate(&self) -> AdditiveIndividual {
        let harmonics = self.harmonics.then(HarmonicsComponent::create);

        let individual = AdditiveIndividual {
            target: Arc::clone(self.target.as_ref()
                .expect("Expected target in AdditiveIndividualGenerator")),
            fitness_type: self.fitness_type,
            fitness: None,
            harmonics,
        };

        individual.include_fitness()
    }

    fn target(mut self, target: Arc<Signal>) -> Self {
        self.target = Some(target);
        self
    }

    fn fitness_type(mut self, fitness_type: FitnessType) -> Self {
        self.fitness_type = fitness_type;
        self
    }

    fn get_target(&self) -> Arc<Signal> {
        Arc::clone(self.target.as_ref().expect("The generator should have a target set."))
    }
}

impl AdditiveIndividualGenerator {

    /// Whether the individual should include a harmonics component.
    pub fn harmonics(mut self) -> Self {
        self.harmonics = true;
        self
    }
}
