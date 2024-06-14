use crate::utils::random_weighted_average;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EnvelopeComponent {
    attack: u32,  // ms
    decay: u32,   // ms
    sustain: u8,  // level 0 - 255
    release: u32, // ms
}

impl EnvelopeComponent {
    pub(crate) fn create() -> Self {
        let mut rng = thread_rng();

        Self {
            attack: rng.gen_range(0..2000),
            decay: rng.gen_range(0..3000),
            sustain: rng.gen_range(0..255) as u8,
            release: rng.gen_range(0..5000),
        }
    }

    pub(crate) fn combine(&self, other: &Self, r: f32) -> Option<Self> {
        let mut rng = thread_rng();

        Some(
            Self {
                attack: random_weighted_average(self.attack as f32, other.attack as f32, r,
                    rng.gen_range(0..2000) as f32,
                ) as u32,
                decay: random_weighted_average(self.decay as f32, other.decay as f32, r,
                    rng.gen_range(0..3000) as f32,
                ) as u32,
                sustain: random_weighted_average(self.sustain as f32, other.sustain as f32, r,
                    rng.gen_range(0..255) as f32,
                ) as u8,
                release: random_weighted_average(self.release as f32, other.release as f32, r,
                    rng.gen_range(0..5000) as f32,
                ) as u32,
            }
        )
    }

    pub(crate) fn evolve(&self, step_size: f32) -> Self {
        todo!()
    }
}
