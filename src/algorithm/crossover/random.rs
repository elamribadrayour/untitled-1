use anyhow::Result;
use rand::Rng;

use crate::algorithm::crossover::Crossover;
use crate::population::{Gene, Individual, Population};

pub struct Random {
    rate: f32,
}

impl Random {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Crossover for Random {
    fn individual(&self, i1: &Individual, i2: &Individual, id: usize) -> Result<Individual> {
        if i1.is_empty() || i1.len() != i2.len() {
            return Err(anyhow::anyhow!("i1 empty or individuals lengths mismatch"));
        }

        let size = i1.len();
        let mut rng = rand::rng();
        let genes = (0..size)
            .map(|i| {
                if rng.random_range(0.0..1.0) < self.rate {
                    i1.genes[i].clone()
                } else {
                    i2.genes[i].clone()
                }
            })
            .collect::<Vec<Gene>>();

        Ok(Individual::genes(&genes, id))
    }

    fn population(&self, population: &Population, size: usize) -> Result<Population> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("random crossover: population is empty"));
        }

        let mut rng = rand::rng();

        let population_size = population.len();
        let individuals = (0..size)
            .map(|id| {
                let id1 = rng.random_range(0..population_size);
                let id2 = rng.random_range(0..population_size);
                let i1 = &population.individuals[id1];
                let i2 = &population.individuals[id2];
                self.individual(i1, i2, id).unwrap()
            })
            .collect::<Vec<Individual>>();

        Ok(Population::individuals(&individuals))
    }
}
