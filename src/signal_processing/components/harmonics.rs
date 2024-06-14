use crate::signal_processing::{LENGTH, SAMPLE_RATE, Signal};
use crate::simulation::components::harmonics::HarmonicsComponent;
use crate::signal_processing::components::oscillator::sine_wave;
type Frequency = f32;
type Amplitude = f32;

impl Signal {
    /// Modifies an existing signal based on the generated parameters.
    pub fn apply_harmonics(&mut self, harmonics: &HarmonicsComponent) {
        let phase_offset = 0.0; // TODO specify

        for (f, a) in generate_harmonics(harmonics.freq, &harmonics.amplitudes) {
            *self = self.add_amp(&sine_wave(f, LENGTH, SAMPLE_RATE as f32, a, phase_offset));
        }
    }
}

pub fn generate_harmonics(freq: Frequency, amplitudes: &[Amplitude]) -> Vec<(Frequency, Amplitude)> {
    amplitudes.iter().enumerate().map(|(i, &a)| (freq * (i+1) as f32, a)).collect()
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::components::harmonics::generate_harmonics;

    #[test]
    fn test_generate_harmonics() {
        let mut pairs = generate_harmonics(440.0, &[0.1, 0.2, 0.4, 0.8]).into_iter();
        assert_eq!(pairs.next(), Some((440.0 * 1f32, 0.1)));
        assert_eq!(pairs.next(), Some((440.0 * 2f32, 0.2)));
        assert_eq!(pairs.next(), Some((440.0 * 3f32, 0.4)));
        assert_eq!(pairs.next(), Some((440.0 * 4f32, 0.8)));
    }
}