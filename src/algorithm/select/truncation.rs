use anyhow::Result;

use crate::algorithm::Select;
use crate::population::Population;

pub struct Truncation {
    pub rate: f32,
}

impl Truncation {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Select for Truncation {
    fn population(&self, population: &Population, fitnesses: &[f32]) -> Result<Population> {
        // https://en.wikipedia.org/wiki/Truncation_selection
        if population.is_empty() || population.len() != fitnesses.len() {
            return Err(anyhow::anyhow!("population is empty or mismatched length"));
        }

        // Sort by fitness
        let mut indices = (0..population.len()).collect::<Vec<usize>>();
        indices.sort_by(|a, b| fitnesses[*b].partial_cmp(&fitnesses[*a]).unwrap());

        // Select the best individuals
        let size = (self.rate * population.len() as f32) as usize;
        let mut new_individuals = Vec::with_capacity(size);
        for i in indices[..size].iter() {
            new_individuals.push(population.individuals[*i].clone());
        }
        let new_population = Population::individuals(new_individuals);

        Ok(new_population)
    }
}
