use anyhow::Result;
use rand::Rng;

use crate::algorithm::Fitness;
use crate::population::Population;

pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fitness for Random {
    fn population(&self, population: &Population) -> Result<Vec<f32>> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("population is empty"));
        }

        let mut rng = rand::rng();
        let fitnesses = (0..population.len())
            .map(|_| rng.random_range(0.0..1.0))
            .collect();
        Ok(fitnesses)
    }
}
