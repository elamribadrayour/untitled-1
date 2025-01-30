use anyhow::Result;
use rand::Rng;

use crate::algorithm::Fitness;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fitness for Random {
    fn individual(&self, _: &Individual, _: &Assets, _: &Grid) -> Result<f32> {
        Ok(rand::rng().random_range(0.0..1.0))
    }

    fn population(&self, population: &Population, _: &Assets, _: &Grid) -> Result<Vec<f32>> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("population is empty"));
        }

        let fitnesses = (0..population.len())
            .map(|_| rand::rng().random_range(0.0..1.0))
            .collect();
        Ok(fitnesses)
    }
}
