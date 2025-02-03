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
    fn population(&self, population: &Population) -> Result<Population> {
        // https://en.wikipedia.org/wiki/Truncation_selection
        if population.is_empty() {
            return Err(anyhow::anyhow!("population is empty"));
        }

        // Sort by fitness
        let mut indices = (0..population.len()).collect::<Vec<usize>>();
        indices.sort_by(|a, b| {
            population.individuals[*b]
                .fitness
                .partial_cmp(&population.individuals[*a].fitness)
                .unwrap()
        });

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
