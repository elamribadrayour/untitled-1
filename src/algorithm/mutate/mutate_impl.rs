use anyhow::Result;

use crate::algorithm::mutate::Random;
use crate::config::MutateConfig;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub trait Mutate {
    fn gene(&self, gene: &Gene, assets: &Assets) -> Result<Gene>;
    fn individual(&self, individual: &Individual, assets: &Assets) -> Result<Individual>;
    fn population(&self, population: &Population, assets: &Assets) -> Result<Population>;
}

pub fn get(config: &MutateConfig) -> Result<Box<dyn Mutate>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new(config.rate)) as Box<dyn Mutate>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
