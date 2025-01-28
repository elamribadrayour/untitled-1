use anyhow::Result;

use crate::algorithm::crossover::Random;
use crate::population::{Individual, Population};

pub trait Crossover {
    fn individual(&self, i1: &Individual, i2: &Individual, id: usize) -> Result<Individual>;
    fn population(&self, population: &Population, size: usize) -> Result<Population>;
}

pub fn get(name: &str, rate: f32) -> Result<Box<dyn Crossover>> {
    match name {
        "random" => Ok(Box::new(Random::new(rate)) as Box<dyn Crossover>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
