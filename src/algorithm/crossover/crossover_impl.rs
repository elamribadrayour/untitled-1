use anyhow::Result;

use crate::algorithm::crossover::Random;
use crate::config::CrossoverConfig;
use crate::population::{Individual, Population};

pub trait Crossover {
    fn individual(&self, i1: &Individual, i2: &Individual, id: usize) -> Result<Individual>;
    fn population(&self, population: &Population, size: usize) -> Result<Population>;
}

pub fn get(config: &CrossoverConfig) -> Result<Box<dyn Crossover>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new(config.rate)) as Box<dyn Crossover>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
