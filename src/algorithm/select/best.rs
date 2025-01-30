use anyhow::Result;

use crate::algorithm::Select;
use crate::population::Population;

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

        let size = (self.rate * population.len() as f32) as usize;
        let individuals = population.individuals[0..size].to_vec();
        let population = Population::individuals(&individuals);
        Ok(population)
    }
}
