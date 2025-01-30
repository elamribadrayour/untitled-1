use anyhow::Result;

use crate::algorithm::fitness::{Random, Uniformity};
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

pub fn get(name: &str) -> Result<Box<dyn Fitness>> {
    match name {
        "random" => Ok(Box::new(Random::new()) as Box<dyn Fitness>),
        "uniformity" => Ok(Box::new(Uniformity::new()) as Box<dyn Fitness>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
