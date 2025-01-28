use anyhow::Result;

use crate::algorithm::mutate::Random;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub trait Mutate {
    fn gene(&self, gene: &Gene, assets: &Assets) -> Result<Gene>;
    fn individual(&self, individual: &Individual, assets: &Assets) -> Result<Individual>;
    fn population(&self, population: &Population, assets: &Assets) -> Result<Population>;
}

pub fn get(name: &str, rate: f32) -> Result<Box<dyn Mutate>> {
    match name {
        "random" => Ok(Box::new(Random::new(rate)) as Box<dyn Mutate>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
