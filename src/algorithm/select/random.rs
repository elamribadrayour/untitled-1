use anyhow::Result;
use rand::Rng;

use crate::algorithm::Select;
use crate::population::{Individual, Population};

pub struct Random;

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Select for Random {
    fn population(&self, population: &Population, _: &[f32]) -> Result<Population> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("population is empty"));
        }

        let size = population.len();
        let mut rng = rand::rng();
        let individuals = (0..size)
            .map(|_| population.individuals[rng.random_range(0..size)].clone())
            .collect::<Vec<Individual>>();
        let population = Population::individuals(individuals);
        Ok(population)
    }
}
