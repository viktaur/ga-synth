use std::fs;
use std::path::Path;
use csv::Writer;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use crate::simulation::algorithms::genetic::{GASimulation, Individual};
use crate::simulation::algorithms::hillclimbing::HillClimbingSimulation;
use crate::utils::{mean, std};

#[derive(Default)]
pub struct Recorder<R: Record> {
    rows: Vec<R>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GenerationRow {
    generation: u32,
    offspring: u32,
    fundamental: f32,
    max_fitness: f32,
    average_fitness: f32,
    std: f32,
}

#[derive(serde::Serialize, Clone, Default)]
pub struct IterationRow {
    iteration: u32,
    fitness: f32,
    fundamental: f32
}

impl<R: Record> Recorder<R> {
    pub(crate) fn new() -> Self {
        Self {
            rows: vec![]
        }
    }

    pub fn add_record(&mut self, record: R) {
        self.rows.push(record);
    }

    pub fn to_csv(&self, file_path: &str) -> Result<(), ()> {
        // fs::create_dir("exports/csv").map_err(|_| ())?;
        let path = Path::new("exports/csv").join(file_path);
        fs::create_dir_all(path.clone().parent().expect("File should have parent."))
            .map_err(|_| ())?;
        let mut wtr = Writer::from_path(path)
            .expect("Writer should have been created from path.");
        for row in &self.rows {
            wtr.serialize(row).expect("Row should have been passed to the CSV writer.");
        }
        
        wtr.flush().expect("Writer should have been flushed.");
        println!("Data successfully written to file {file_path}");
        Ok(())
    }
}

pub trait Record: Serialize {}

impl Record for GenerationRow {}
impl Record for IterationRow {}

impl<T: Individual> From<&mut GASimulation<T>> for GenerationRow {
    fn from(simulation: &mut GASimulation<T>) -> Self {
        let generation = simulation.generation;
        let offspring = simulation.offspring;
        let fundamental = simulation.fundamental.unwrap_or(0.0);
            // .expect("There should be a fundamental frequency");
        let max_fitness = simulation.population
            .first()
            .expect("There should be at least one individual")
            .fitness();
        let average_fitness = mean(&simulation.population.iter().map(|i| i.fitness()).collect_vec());
        let std = std(&simulation.population.iter().map(|i| i.fitness()).collect_vec());

        Self {
            generation,
            offspring,
            fundamental,
            max_fitness,
            average_fitness,
            std
        }
    }
}

impl<T: Individual> From<&mut HillClimbingSimulation<T>> for IterationRow {
    fn from(simulation: &mut HillClimbingSimulation<T>) -> Self {
        let iteration = simulation.iteration;
        let fitness = simulation.current_individual.fitness();
        let fundamental = simulation.current_individual.get_fundamental().unwrap_or(0.0);
        
        Self {
            iteration,
            fitness,
            fundamental,
        }
    }
}

#[cfg(test)]
mod tests {
    use bincode::Options;
    use super::*;

    #[test]
    fn test_csv_export() {
        let path = "tests/test.csv";

        // Write
        let mut recorder = Recorder::new();
        let record = GenerationRow { generation: 10, max_fitness: 0.3, average_fitness: 0.3, std: 0.3, offspring: 50, fundamental: 0.0 };
        recorder.add_record(record.clone());
        recorder.add_record(record.clone());
        recorder.add_record(record.clone());
        recorder.to_csv(path).unwrap();

        // Verify
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(Path::new("exports/csv").join(path))
            .unwrap();
        let mut iter = rdr.deserialize();

        let rd_record: GenerationRow = iter.next().unwrap().unwrap();
        assert_eq!(rd_record, record);
    }
}