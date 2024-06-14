use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::simulation::algorithms::hillclimbing::evolve_value;
use crate::utils::random_weighted_average;

const MIN_FREQ: f32 = 20.0;
const MAX_FREQ: f32 = 10_000.0;

/// Represents the component containing the harmonics information in additive synthesis.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HarmonicsComponent {
    /// Fundamental frequency of the harmonic series.
    pub freq: f32,
    /// Amplitudes of each of the n harmonics.
    pub amplitudes: Vec<f32>
}

impl HarmonicsComponent {

    pub(crate) fn create() -> Self {
        let mut rng = thread_rng();
        let freq = Self::random_freq(&mut rng);
        let n = 9;
        let amplitudes = (0..n).map(|_| rng.gen()).collect();

        Self {
            freq,
            amplitudes
        }
    }

    pub(crate) fn combine(&self, other: &Self, r: f32) -> Option<Self> where Self: Sized {
        let mut rng = thread_rng();

        let freq = random_weighted_average(self.freq, other.freq, r, Self::random_freq(&mut rng));
        let amplitudes = self.amplitudes.iter().zip(&other.amplitudes).map(|(&s, &o)| {
            random_weighted_average(s, o, r, rng.gen())
        }).collect();

        Some(
            Self {
                freq,
                amplitudes
            }
        )
    }

    pub(crate) fn evolve(&self, step_size: f32) -> Self {
        let mut rng = thread_rng();

        Self {
            freq: evolve_value(self.freq, 20.0, 10_000.0, step_size, &mut rng),
            amplitudes: self.amplitudes.iter().map(|&a| evolve_value(a, 0.0, 1.0, step_size, &mut rng)).collect()
        }
    }

    fn random_freq(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_FREQ..MAX_FREQ)
    }
}