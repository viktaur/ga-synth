pub mod signal_analysis;
pub mod components;

use std::fs;
use std::fs::File;
use std::iter::zip;
use std::path::Path;
use crate::error::SignalProcessingError;
use crate::error::SignalProcessingError::CouldNotReadFromFile;
use anyhow::Result;

// const FREQ: f32 = 440.0;
pub const LENGTH: f32 = 3.0;
pub const SAMPLE_RATE: u32 = 44_100;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Signal(Vec<f32>);

impl IntoIterator for Signal {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Signal {
    pub fn init(length: f32, sample_rate: f32) -> Self {
        Signal((0..(length * sample_rate) as u32).map(|_| 0.0).collect())
    }

    pub fn from_samples(samples: &[f32]) -> Self {
        Signal(samples.into())
    }

    pub fn from_wav_file(file: File) -> Result<Self, SignalProcessingError> {
        let (_, samples) = wav_io::read_from_file(file).map_err(CouldNotReadFromFile)?;
        Ok(Signal(samples))
    }

    pub fn add_amp(&self, other: &Self) -> Self {
        Signal(zip(&self.0, &other.0).map(|(&s, &o)| s + o).collect())
    }

    pub fn scale_amp(&self, factor: f32) -> Self {
        Signal(self.0.iter().map(|s| s * factor).collect())
    }

    // TODO use custom errors
    /// Exports the signal to a WAV file using the wav_io crate.
    pub fn to_wav(&self, file_path: &str) -> Result<(), ()> {
        // fs::create_dir("exports/signal").map_err(|_| ())?;
        let path = Path::new("exports/signal").join(file_path);
        fs::create_dir_all(path.clone().parent().expect("File should have parent."))
            .map_err(|_| ())?;
        let head = wav_io::new_mono_header();
        let mut file_out = File::create(path)
            .expect("The creation of a new file should be successful");
        wav_io::write_to_file(&mut file_out, &head, &self.0).map_err(|_| ())?;
        println!("Signal successfully written to file {}", file_path);
        Ok(())
    }

    pub fn n_samples(&self) -> usize {
        self.0.len()
    }

    pub fn samples(&self) -> &[f32] {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn synth_signal_from_basic() {
    //     todo!()
    // }
}
