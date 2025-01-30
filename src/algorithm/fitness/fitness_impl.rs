use anyhow::Result;

use crate::algorithm::fitness::{Random, StrictUniformity};
use crate::config::FitnessConfig;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub trait Fitness {
    fn individual(&self, individual: &Individual, assets: &Assets, grid: &Grid) -> Result<f32>;
    fn population(&self, population: &Population, assets: &Assets, grid: &Grid)
        -> Result<Vec<f32>>;
}

pub fn get(config: &FitnessConfig) -> Result<Box<dyn Fitness>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new()) as Box<dyn Fitness>),
        "strict-uniformity" => Ok(Box::new(StrictUniformity::new()) as Box<dyn Fitness>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
