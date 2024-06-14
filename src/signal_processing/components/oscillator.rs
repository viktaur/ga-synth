use crate::simulation::components::oscillator::OscillatorComponent;
use crate::signal_processing::{Signal, LENGTH, SAMPLE_RATE};

impl Signal {
    pub fn apply_oscillator(&mut self, oscillator: OscillatorComponent) {
        let sine = sine_wave(
            oscillator.freq,
            LENGTH,
            SAMPLE_RATE as f32,
            oscillator.sine_amp,
            oscillator.sine_phase,
        );
        let square = square_wave(
            oscillator.freq,
            LENGTH,
            SAMPLE_RATE as f32,
            oscillator.square_amp,
            oscillator.square_phase,
        );
        let saw = saw_wave(
            oscillator.freq,
            LENGTH,
            SAMPLE_RATE as f32,
            oscillator.saw_amp,
            oscillator.saw_phase,
        );

        // *self = sine.add_amp(&square).add_amp(&saw).scale_amp(1.0 / 3.0);
        *self = sine.add_amp(&square).add_amp(&saw);
    }
}

/// Produces a sine waveform with the specified parameters.
pub fn sine_wave(
    freq: f32,
    length: f32,
    sample_rate: f32,
    amplitude: f32,
    phase_offset: f32
) -> Signal {
    const PI_2: f32 = core::f32::consts::PI * 2.0;

    let sample_period = 1.0 / sample_rate;
    let n = sample_rate * length;

    let mut samples = vec![];

    for i in 0..n as u32 {
        samples.push(
            amplitude * f32::sin(PI_2 * freq * i as f32 * sample_period + phase_offset),
        );
    }

    Signal(samples)
}

/// Produces a square waveform with the specified parameters.
pub fn square_wave(
    freq: f32,
    length: f32,
    sample_rate: f32,
    amplitude: f32,
    phase_offset: f32
) -> Signal {
    const PI_2: f32 = core::f32::consts::PI * 2.0;

    let samples_cycle = sample_rate / freq;
    let phase_factor = samples_cycle / PI_2;
    let n = sample_rate * length;

    let mut samples: Vec<f32> = vec![];

    for i in 0..n as u32 {
        let value =
            if ((i as f32 + (phase_factor * phase_offset)) % samples_cycle) < (samples_cycle / 2.0) {
                1
            } else {
                -1
            };

        samples.push(amplitude * value as f32);
    }

    Signal(samples)
}

/// Produces a saw waveform at the specified parameters.
pub fn saw_wave(
    freq: f32,
    length: f32,
    sample_rate: f32,
    amplitude: f32,
    phase_offset: f32
) -> Signal {
    const PI_2: f32 = core::f32::consts::PI * 2.0;

    let sample_period = 1.0 / sample_rate;
    let phase_factor = sample_rate / (freq * PI_2);
    let n = sample_rate * length;

    let mut samples: Vec<f32> = vec![];

    for i in 0..n as u32 {
        let value: f32 = freq
            * (((i as f32 + (phase_factor * phase_offset)) * sample_period)
                % (1.0 / freq)) * 2.0 - 1.0;

        samples.push(amplitude * value);
    }

    Signal(samples)
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::components::oscillator::{saw_wave, sine_wave, square_wave};

    #[test]
    fn test_sine() {
        let mut signal = sine_wave(1.0, 1.0, 4.0, 1.0, 0.0).into_iter();
        assert_eq!(signal.next(), Some(0.0));
        assert_eq!(signal.next(), Some(1.0));
        signal.next();
        assert_eq!(signal.next(), Some(-1.0));
    }

    #[test]
    fn test_square() {
        let mut signal = square_wave(1.0, 1.0, 4.0, 1.0, 0.0).into_iter();
        assert_eq!(signal.next(), Some(1.0));
        assert_eq!(signal.next(), Some(1.0));
        assert_eq!(signal.next(), Some(-1.0));
        assert_eq!(signal.next(), Some(-1.0));
    }

    #[test]
    fn test_saw() {
        let mut signal = saw_wave(1.0, 2.0, 4.0, 1.0, 0.0).into_iter();
        assert_eq!(signal.next(), Some(-1.0));
        assert_eq!(signal.next(), Some(-0.5));
        assert_eq!(signal.next(), Some(0.0));
        assert_eq!(signal.next(), Some(0.5));
    }
}
