use std::char::MAX;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use crate::simulation::algorithms::hillclimbing::evolve_value;
use crate::utils::random_weighted_average;

const MIN_FREQ: f32 = 0.0;
const MAX_FREQ: f32 = 20_000.0;
const MIN_BAND: f32 = 0.01;
const MAX_BAND: f32 = 4.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum FilterComponent {
    LowPass {
        cutoff_freq: f32,
        band: f32,
    },
    HighPass {
        cutoff_freq: f32,
        band: f32,
    },
    BandPass {
        low_freq: f32,
        high_freq: f32,
        band: f32
    },
    BandReject {
        low_freq: f32,
        high_freq: f32,
        band: f32
    },
}

#[derive(Copy, Clone)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    BandReject
}

impl FilterComponent {
    pub(crate) fn create(filter_type: FilterType) -> Self {
        let mut rng = thread_rng();

        match filter_type {
            FilterType::LowPass => {
                Self::LowPass {
                    cutoff_freq: Self::random_freq(&mut rng),
                    band: Self::random_band(&mut rng),
                }
            }
            FilterType::HighPass => {
                Self::HighPass {
                    cutoff_freq: Self::random_freq(&mut rng),
                    band: Self::random_band(&mut rng),
                }
            }
            FilterType::BandPass => {
                let (freq_1, freq_2) = (Self::random_freq(&mut rng), Self::random_freq(&mut rng));

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = Self::random_band(&mut rng);

                Self::BandPass {
                    low_freq,
                    high_freq,
                    band,
                }
            }
            FilterType::BandReject => {
                let (freq_1, freq_2) = (Self::random_freq(&mut rng), Self::random_freq(&mut rng));

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = Self::random_band(&mut rng);

                Self::BandReject {
                    low_freq,
                    high_freq,
                    band,
                }
            }
        }
    }

    pub(crate) fn combine(&self, other: &Self, mutation_rate: f32) -> Option<Self> {
        let mut rng = thread_rng();

        match (self, other) {
            (
                Self::LowPass {
                    cutoff_freq: self_cutoff_freq, band: self_band
                },
                Self::LowPass {
                    cutoff_freq: other_cutoff_freq, band: other_band
                }
            ) => {
                Some(
                    Self::LowPass {
                        cutoff_freq: random_weighted_average(
                            *self_cutoff_freq,
                            *other_cutoff_freq,
                            mutation_rate,
                            Self::random_freq(&mut rng)
                        ),
                        band: random_weighted_average(
                            *self_band,
                            *other_band,
                            mutation_rate,
                            Self::random_band(&mut rng)
                        )
                    }
                )
            },

            (
                Self::HighPass {
                    cutoff_freq: self_cutoff_freq, band: self_band
                },
                Self::HighPass {
                    cutoff_freq: other_cutoff_freq, band: other_band
                }
            ) => {
                Some(
                    Self::HighPass {
                        cutoff_freq: random_weighted_average(
                            *self_cutoff_freq,
                            *other_cutoff_freq,
                            mutation_rate,
                            Self::random_freq(&mut rng)
                        ),
                        band: random_weighted_average(
                            *self_band,
                            *other_band,
                            mutation_rate,
                            Self::random_band(&mut rng)
                        )
                    }
                )
            },

            (
                Self::BandPass {
                    low_freq: self_low_freq, high_freq: self_high_freq, band: self_band
                },
                Self::BandPass {
                    low_freq: other_low_freq, high_freq: other_high_freq, band: other_band
                }
            ) => {
                // We don't know which of the generated frequencies is going to be higher, so we will
                // re-assign the low and high frequency bounds once both are generated.
                let freq_1 = random_weighted_average(
                    *self_low_freq,
                    *other_low_freq,
                    mutation_rate,
                    Self::random_freq(&mut rng)
                );

                let freq_2 = random_weighted_average(
                    *self_high_freq,
                    *other_high_freq,
                    mutation_rate,
                    Self::random_freq(&mut rng)
                );

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = random_weighted_average(
                    *self_band,
                    *other_band,
                    mutation_rate,
                    Self::random_band(&mut rng)
                );

                Some(
                    Self::BandPass {
                        low_freq,
                        high_freq,
                        band,
                    }
                )
            },

            (
                Self::BandReject {
                    low_freq: self_low_freq, high_freq: self_high_freq, band: self_band
                },
                Self::BandReject {
                    low_freq: other_low_freq, high_freq: other_high_freq, band: other_band
                }
            ) => {
                let freq_1 = random_weighted_average(
                    *self_low_freq,
                    *other_low_freq,
                    mutation_rate,
                    Self::random_freq(&mut rng)
                );

                let freq_2 = random_weighted_average(
                    *self_high_freq,
                    *other_high_freq,
                    mutation_rate,
                    Self::random_freq(&mut rng)
                );

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = random_weighted_average(
                    *self_band,
                    *other_band,
                    mutation_rate,
                    Self::random_band(&mut rng)
                );

                Some(
                    Self::BandReject {
                        low_freq,
                        high_freq,
                        band,
                    }
                )
            },
            _ => None
        }
    }

    pub(crate) fn evolve(&self, step_size: f32) -> Self {
        let mut rng = thread_rng();

        match self {
            FilterComponent::LowPass { cutoff_freq, band } => {
                Self::LowPass {
                    cutoff_freq: evolve_value(*cutoff_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng),
                    band: evolve_value(*band, MIN_BAND, MAX_BAND, step_size, &mut rng),
                }
            }
            FilterComponent::HighPass { cutoff_freq, band} => {
                Self::HighPass {
                    cutoff_freq: evolve_value(*cutoff_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng),
                    band: evolve_value(*band, MIN_BAND, MAX_BAND, step_size, &mut rng),
                }
            }
            FilterComponent::BandPass { low_freq, high_freq, band } => {
                let freq_1 = evolve_value(*low_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng);
                let freq_2 = evolve_value(*high_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng);

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = evolve_value(*band, MIN_BAND, MAX_BAND, step_size, &mut rng);

                Self::BandPass {
                    low_freq,
                    high_freq,
                    band
                }
            }
            FilterComponent::BandReject { low_freq, high_freq, band } => {
                let freq_1 = evolve_value(*low_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng);
                let freq_2 = evolve_value(*high_freq, MIN_FREQ, MAX_FREQ, step_size, &mut rng);

                let (low_freq, high_freq) = if freq_1 < freq_2 {
                    (freq_1, freq_2)
                } else {
                    (freq_2, freq_1)
                };

                let band = evolve_value(*band, MIN_BAND, MAX_BAND, step_size, &mut rng);

                Self::BandReject {
                    low_freq,
                    high_freq,
                    band
                }
            }
        }
    }

    fn random_freq(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_FREQ..MAX_FREQ)
    }

    fn random_band(rng: &mut ThreadRng) -> f32 {
        rng.gen_range(MIN_BAND..MAX_BAND)
    }
}