use crate::signal_processing::Signal;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use crate::{FitnessType};
use crate::simulation::algorithms::genetic::{GASimulation, Individual, IndividualGenerator};
use crate::simulation::components::envelope::EnvelopeComponent;
use crate::simulation::components::filters::{FilterComponent, FilterType};
use crate::simulation::components::oscillator::OscillatorComponent;

/// Contains the components and other information related to an individual representing subtractive
/// synthesis.
#[derive(Clone, Debug, PartialEq)]
pub struct SubtractiveIndividual {
    target: Arc<Signal>,
    fitness_type: FitnessType,
    fitness: Option<f32>,
    oscillator: Option<OscillatorComponent>,
    envelope: Option<EnvelopeComponent>,
    filter: Option<FilterComponent>
}

/// Specifies the components of a SubtractiveIndividual and other information.
#[derive(Clone)]
pub struct SubtractiveIndividualGenerator {
    target: Option<Arc<Signal>>,
    fitness_type: FitnessType,
    oscillator: bool,
    envelope: bool,
    filter: Option<FilterType>,
}

impl Individual for SubtractiveIndividual {
    type Generator = SubtractiveIndividualGenerator;

    fn new_generator() -> Self::Generator {
        Self::Generator::new()
    }

    fn get_target(&self) -> Arc<Signal> {
        Arc::clone(&self.target)
    }

    fn fitness(&self) -> f32 {
        self.fitness.unwrap_or_else(|| self.calculate_fitness())
    }

    fn get_fitness_type(&self) -> FitnessType {
        self.fitness_type
    }

    fn include_fitness(mut self) -> Self {
        self.fitness = Some(self.calculate_fitness());
        self
    }

    fn crossover(&self, other: &Self, r: f32) -> Option<Self> {
        let oscillator = match (&self.oscillator, &other.oscillator) {
            (Some(s), Some(o)) => s.combine(o, r),
            _ => None,
        };
        
        let envelope = match (&self.envelope, &other.envelope) {
            (Some(s), Some(o)) => s.combine(o, r),
            _ => None,
        };
        
        let filter = match (&self.filter, &other.filter) {
            (Some(s), Some(o)) => s.combine(o, r),
            _ => None,
        };
        
        let offspring = Self {
            fitness_type: self.fitness_type,
            fitness: None,
            target: self.get_target(),
            oscillator,
            envelope,
            filter,
        };

        Some(offspring.include_fitness())
    }

    /// Converts a genetic individual to a `Signal` by applying the specified components.
    fn to_signal(&self) -> Signal {
        let mut signal = Signal::default();

        if let Some(oscillator) = self.oscillator {
            signal.apply_oscillator(oscillator);
        }

        if let Some(envelope) = self.envelope {
            signal.apply_envelope(envelope);
        }

        if let Some(filter) = self.filter {
            signal.apply_filter(filter);
        }

        signal
    }

    fn evolve(&self, step_size: f32) -> Self {
        Self {
            target: Arc::clone(&self.target),
            fitness_type: self.fitness_type,
            fitness: None,
            oscillator: self.oscillator.map(|osc| osc.evolve(step_size)),
            envelope: self.envelope.map(|env| env.evolve(step_size)),
            filter: self.filter.map(|fil| fil.evolve(step_size))
        }.include_fitness()
    }

    fn dbg(&self) -> String {
        format!("FITNESS: {:?}, Oscillator: {:?}, Envelope: {:?}, Filter: {:?}",
                self.fitness.unwrap_or(0.0), self.oscillator, self.envelope, self.filter
        )
    }
    
    fn get_fundamental(&self) -> Option<f32> {
        Some(self.oscillator?.freq)
    }
}

impl PartialOrd<Self> for SubtractiveIndividual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SubtractiveIndividual {}

impl Ord for SubtractiveIndividual {
    fn cmp(&self, other: &Self) -> Ordering {
        // Needs to use partial_cmp since f32 does not implement the Ord trait.
        self.fitness().partial_cmp(&other.fitness()).expect("No fitness value should be NaN.")
    }
}

impl IndividualGenerator<SubtractiveIndividual> for SubtractiveIndividualGenerator {
    fn new() -> Self {
        SubtractiveIndividualGenerator {
            target: None,
            fitness_type: FitnessType::default(),
            oscillator: false,
            envelope: false,
            filter: None,
        }
    }

    fn generate(&self) -> SubtractiveIndividual {
        let oscillator = self.oscillator.then(OscillatorComponent::create);
        let envelope = self.envelope.then(EnvelopeComponent::create);
        let filter = self.filter.as_ref().map(|&f| FilterComponent::create(f));

        let individual = SubtractiveIndividual {
            target: Arc::clone(self.target.as_ref()
                .expect("Expected target in SubtractiveIndividualGenerator")),
            fitness_type: self.fitness_type,
            fitness: None,
            oscillator,
            envelope,
            filter,
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

impl SubtractiveIndividualGenerator {

    /// Used to specify whether the individual will contain an oscillator component.
    pub fn oscillator(mut self) -> Self {
        self.oscillator = true;
        self
    }

    /// Used to specify whether the individual will contain an envelope component.
    pub fn envelope(mut self) -> Self {
        self.envelope = true;
        self
    }

    /// uSed to specify whether the individual will contain a filter component and its type.
    pub fn filter(mut self, filter_type: FilterType) -> Self {
        self.filter = Some(filter_type);
        self
    }
}
