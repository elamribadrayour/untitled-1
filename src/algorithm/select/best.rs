use anyhow::Result;

use crate::algorithm::Select;
use crate::population::{Individual, Population};

pub struct Best {
    pub rate: f32,
}

impl Best {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Select for Best {
    fn population(&self, population: &Population, fitnesses: &[f32]) -> Result<Population> {
        if population.is_empty() || population.len() != fitnesses.len() {
            return Err(anyhow::anyhow!("population is empty or mismatched length"));
        }

        let mut individuals = population
            .individuals
            .iter()
            .zip(fitnesses)
            .collect::<Vec<(&Individual, &f32)>>();
        individuals
            .sort_by(|(_, &f1), (_, &f2)| f2.partial_cmp(&f1).unwrap_or(std::cmp::Ordering::Equal));
        let individuals = individuals
            .iter()
            .map(|(i, _)| (*i).clone())
            .collect::<Vec<Individual>>();

        let size = (self.rate * population.len() as f32) as usize;
        let individuals = &individuals[0..size];
        let population = Population::individuals(individuals);
        Ok(population)
    }
}
