use std::f32::consts::PI;
use crate::signal_processing::{SAMPLE_RATE, Signal};
use crate::simulation::components::filters::FilterComponent;
use crate::utils;

impl Signal {

    pub(crate) fn apply_filter(&mut self, filter_comp: FilterComponent) {
        let filter = match filter_comp {
            FilterComponent::LowPass { cutoff_freq, band } => {
                Self::low_pass_filter(cutoff_freq, band)
            }
            FilterComponent::HighPass { cutoff_freq, band } => {
                Self::high_pass_filter(cutoff_freq, band)
            }
            FilterComponent::BandPass { low_freq: low_frequency, high_freq: high_frequency, band } => {
                Self::band_pass_filter(low_frequency, high_frequency, band)
            }
            FilterComponent::BandReject { low_freq: low_frequency, high_freq: high_frequency, band } => {
                Self::band_reject_filter(low_frequency, high_frequency, band)
            }
        };

        *self = Signal::from_samples(&utils::convolve(&filter, self.samples()))
    }

    fn low_pass_filter(cutoff_freq: f32, band: f32) -> Vec<f32> {
        let cutoff = Self::cutoff_from_frequency(cutoff_freq);

        // Filter length, i.e. the number of points in the filter. Inversely proportional to the
        // bandwidth.
        let mut n = (4.0 / band).ceil() as usize;
        if n % 2 == 1 {
            n += 1;
        }

        let sinc = |x: f32| -> f32 { (x * PI).sin() / (x * PI) };

        let sinc_wave: Vec<f32> = (0..n)
            .map(|i| sinc(2.0 * cutoff * (i as f32 - (n as f32 - 1.0) / 2.0)))
            .collect();

        let blackman_window = utils::blackman_window(n);

        let filter: Vec<f32> = sinc_wave
            .iter()
            .zip(blackman_window.iter())
            .map(|tup| *tup.0 * *tup.1)
            .collect();

        // Normalize
        let sum = filter.iter().fold(0.0, |acc, &el| acc + el);

        filter.iter().map(|&el| el / sum).collect()
    }

    fn high_pass_filter(cutoff: f32, band: f32) -> Vec<f32> {
        utils::spectral_invert(&Self::low_pass_filter(cutoff, band))
    }

    fn band_pass_filter(low_freq: f32, high_freq: f32, band: f32) -> Vec<f32> {
        assert!(low_freq <= high_freq);
        let low_pass = Self::low_pass_filter(high_freq, band);
        let high_pass = Self::high_pass_filter(low_freq, band);
        utils::add(&high_pass, &low_pass)
    }

    fn band_reject_filter(low_freq: f32, high_freq: f32, band: f32) -> Vec<f32> {
        assert!(low_freq <= high_freq);
        let low_pass = Self::low_pass_filter(low_freq, band);
        let high_pass = Self::high_pass_filter(high_freq, band);
        utils::convolve(&high_pass, &low_pass)
    }

    fn cutoff_from_frequency(freq: f32) -> f32 {
        freq / SAMPLE_RATE as f32
    }
}