use crate::signal_processing::{Signal, SAMPLE_RATE};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};
use std::ops::Sub;
use spectrum_analyzer::error::SpectrumAnalyzerError;
use crate::error::SignalProcessingError;

impl Signal {

    /// Calculates the mean-squared error (MSE) between the frequency spectrum of two signals.
    pub fn freq_spectrum_mse(&self, other: &Self) -> Result<f32, SignalProcessingError> {
        let self_spectrum = self.freq_spectrum()?;
        let other_spectrum = other.freq_spectrum()?;

        // self_spectrum.data().iter().zip(other_spectrum.data().iter()).for_each(|(s, o)| {
        //     println!("self: {:?}, other: {:?}", s, o);
        // });

        // number of discrete frequency points
        let n = self_spectrum.data().len() as f32;

        let self_freq_vals = self_spectrum.data().iter().map(|(f, fv)| fv);
        let other_freq_vals = other_spectrum.data().iter().map(|(f, fv)| fv);

        Ok(
            // perform the mean squared error of the frequency spectrum
            self_freq_vals.zip(other_freq_vals)
                .map(|(s, o)| (s.val() - o.val()).powi(2))
                .sum::<f32>() / n
        )
    }

    pub fn freq_spectrum(&self) -> Result<FrequencySpectrum, SignalProcessingError> {
        samples_fft_to_spectrum(
            self.normalise().samples(),
            SAMPLE_RATE,
            FrequencyLimit::All,
            Some(&|val, info| val - info.min),
        ).map_err(SignalProcessingError::InvalidSpectrum)
    }

    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        self.samples().iter().zip(other.samples()).map(|(s, o)| (s - o).powi(2))
            .sum::<f32>().sqrt()
    }

    /// Creates a copy of the signal whose number of samples is a power of two in order to analyse its frequency spectrum.
    /// Currently not in use
    pub fn extend_pow_two(&self) -> Self {
        let mut samples = self.0.clone();
        let extra = samples.len().next_power_of_two() - samples.len();

        if extra > 0 {
            samples.extend((0..extra).map(|_| 0.0))
        }

        Signal(samples)
    }

    pub fn normalise(&self) -> Self {
        let n = 16_384;
        let mut new_samples = self.0.clone();

        if self.n_samples() >= n {
            Signal(new_samples.into_iter().take(n).collect())
        } else {
            new_samples.extend((0..n - self.n_samples()).map(|_| 0.0));
            Signal(new_samples)
        }
    }
}

// Function that can calculate the error between two Fourier transforms
// pub fn mse()

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use itertools::Itertools;
    use wav_io::splitter::normalize_f;

    #[test]
    fn test_freq_spectrum() {
        let audio_sample = File::open("audio_samples/440hz_sine.wav").unwrap();
        let signal = Signal::from_wav_file(audio_sample).unwrap();
        // let hann_window = SynthSignal::from_samples(hann_window(&signal.0));
        signal
            .freq_spectrum().unwrap()
            .data()
            .iter()
            .for_each(|(fr, fr_val)| println!("{}Hz => {}", fr, fr_val))
    }

    #[test]
    fn test_extend_pow_two() {
        let signal_1 = Signal::from_samples(&[0.0; 5]);
        let signal_2 = Signal::from_samples(&[0.0; 8]);

        assert_eq!(signal_1.extend_pow_two().n_samples(), 8);
        assert_eq!(signal_2.extend_pow_two().n_samples(), 8);

        let audio_sample = File::open("audio_samples/440hz_sine.wav").unwrap();
        let signal = Signal::from_wav_file(audio_sample)
            .unwrap()
            .extend_pow_two();
        println!("{}", signal.n_samples());

        assert!(signal.n_samples().is_power_of_two());
    }

    #[test]
    fn test_normalise() {
        let signal_1 = Signal::from_samples(&(0..18_000).map(|_| 0.5).collect_vec());
        let signal_2 = Signal::from_samples(&(0..1_000).map(|_| 0.5).collect_vec());
        assert_eq!(signal_1.normalise().n_samples(), signal_2.normalise().n_samples());
        assert_eq!(signal_2.normalise().0.last(), Some(&0.0));

        let n = signal_1.normalise().n_samples();
        let signal_3 = Signal::from_samples(&(0..n).map(|_| 0.5).collect_vec());
        assert_eq!(signal_3.n_samples(), signal_3.normalise().n_samples());
    }

    #[test]
    fn test_euclidean_distance() {
        let signal_1 = Signal::from_samples(&[0.0, 0.5, 0.5, 1.0]);
        let signal_2 = Signal::from_samples(&[1.0, 0.0, 1.0, 0.0]);
        assert_eq!(signal_1.euclidean_distance(&signal_2), 2.5f32.sqrt())
    }
}
