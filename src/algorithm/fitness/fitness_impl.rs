use anyhow::Result;

use crate::algorithm::fitness::StrictUniformity;
use crate::config::FitnessConfig;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub trait Fitness {
    fn individual(&self, individual: &mut Individual, assets: &Assets, grid: &Grid) -> Result<()>;
    fn population(&self, population: &mut Population, assets: &Assets, grid: &Grid) -> Result<()>;
}

pub fn get(config: &FitnessConfig) -> Result<Box<dyn Fitness>> {
    match config.name.as_str() {
        "strict-uniformity" => Ok(Box::new(StrictUniformity) as Box<dyn Fitness>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
