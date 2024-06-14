use crate::utils::random_weighted_average;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use crate::simulation::algorithms::hillclimbing::evolve_value;

const MIN_FREQ: f32 = 20.0;
const MAX_FREQ: f32 = 10_000.0;
const MIN_AMP: f32 = 0.0;
const MAX_AMP: f32 = 1.0;
const MIN_PHASE: f32 = 0.0;
const MAX_PHASE: f32 = 2.0 * PI;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct OscillatorComponent {
    pub freq: f32,
    pub sine_amp: f32,
    pub sine_phase: f32,
    pub square_amp: f32,
    pub square_phase: f32,
    pub saw_amp: f32,
    pub saw_phase: f32,
}

impl OscillatorComponent {
    pub(crate) fn create() -> Self {
        let mut rng = thread_rng();

        Self {
            freq: Self::random_freq(&mut rng),
            sine_amp: Self::random_sine_amp(&mut rng),
            sine_phase: Self::random_sine_phase(&mut rng),
            square_amp: Self::random_square_amp(&mut rng),
            square_phase: Self::random_square_phase(&mut rng),
            saw_amp: Self::random_saw_amp(&mut rng),
            saw_phase: Self::random_saw_phase(&mut rng),
        }
    }

    pub(crate) fn combine(&self, other: &Self, mutation_rate: f32) -> Option<Self> {
        let mut rng = thread_rng();

        Some(
            Self {
                freq: random_weighted_average(self.freq, other.freq, mutation_rate, Self::random_freq(&mut rng)),
                sine_amp: random_weighted_average(self.sine_amp, other.sine_amp, mutation_rate, Self::random_sine_amp(&mut rng)),
                sine_phase: random_weighted_average(self.sine_phase, other.sine_phase, mutation_rate, Self::random_sine_phase(&mut rng)),
                square_amp: random_weighted_average(self.square_amp, other.square_amp, mutation_rate, Self::random_square_amp(&mut rng)),
                square_phase: random_weighted_average(self.square_phase, other.square_phase, mutation_rate, Self::random_square_phase(&mut rng)),
                saw_amp: random_weighted_average(self.saw_amp, other.saw_amp, mutation_rate, Self::random_saw_amp(&mut rng)),
                saw_phase: random_weighted_average(self.saw_phase, other.saw_phase, mutation_rate, Self::random_saw_phase(&mut rng)),
            }
        )
    }

    pub(crate) fn evolve(&self, step_size: f32) -> Self {
        let mut rng = thread_rng();

        Self {
            // freq: self.freq + Self::random_freq(&mut rng) * step_size,
            freq: evolve_value(self.freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng),
            sine_amp: evolve_value(self.sine_amp, MIN_AMP, MAX_AMP, step_size, &mut rng),
            sine_phase: evolve_value(self.sine_phase, MIN_PHASE, MAX_PHASE, step_size, &mut rng),
            square_amp: evolve_value(self.square_amp, MIN_AMP, MAX_AMP, step_size, &mut rng),
            square_phase: evolve_value(self.square_phase, MIN_PHASE, MAX_PHASE, step_size, &mut rng),
            saw_amp: evolve_value(self.saw_amp, MIN_AMP, MAX_AMP, step_size, &mut rng),
            saw_phase: evolve_value(self.saw_amp, MIN_PHASE, MAX_PHASE, step_size, &mut rng),
        }
    }
}

impl OscillatorComponent {
    fn random_freq(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_FREQ..MAX_FREQ)
    }

    fn random_sine_amp(rng: &mut ThreadRng) -> f32 {
        rng.gen()
    }

    fn random_sine_phase(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_PHASE..MAX_PHASE)
    }

    fn random_square_amp(rng: &mut ThreadRng) -> f32 {
        rng.gen()
    }

    fn random_square_phase(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_PHASE..MAX_PHASE)
    }

    fn random_saw_amp(rng: &mut ThreadRng) -> f32 {
        rng.gen()
    }

    fn random_saw_phase(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_PHASE..MAX_PHASE)
    }

    // pub(crate) fn reg(&self) -> f32 {
    //     1.0 - (self.sine_amp.powi(2) + self.saw_amp.powi(2) + self.square_amp.powi(2))
    // }
}
