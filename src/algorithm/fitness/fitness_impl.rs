use anyhow::Result;

use crate::algorithm::fitness::{Random, Uniformity};
use crate::config::FitnessConfig;
use crate::population::{Gene, Individual, Population};
use crate::utils::{Assets, Grid};

pub trait Fitness {
    fn gene(
        &self,
        gene: &Gene,
        individual: &Individual,
        assets: &Assets,
        grid: &Grid,
    ) -> Result<f32>;
    fn individual(&self, individual: &Individual, assets: &Assets, grid: &Grid) -> Result<f32>;
    fn population(&self, population: &Population, assets: &Assets, grid: &Grid)
        -> Result<Vec<f32>>;
}

pub fn get(config: &FitnessConfig) -> Result<Box<dyn Fitness>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new()) as Box<dyn Fitness>),
        "uniformity" => Ok(Box::new(Uniformity::new(config.range)) as Box<dyn Fitness>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
