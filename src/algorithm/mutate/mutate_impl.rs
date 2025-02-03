use anyhow::Result;

use crate::algorithm::mutate::{Adaptive, Directed, Random};
use crate::config::MutateConfig;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub trait Mutate {
    fn gene(&self, gene: &Gene, rng: (f32, usize)) -> Result<Gene>;
    fn individual(&self, individual: &Individual, rngs: &[(f32, usize)]) -> Result<Individual>;
    fn population(&self, population: &Population, assets: &Assets) -> Result<Population>;

    fn update(&mut self, fitness: f32);
    fn current_rate(&self) -> f32;
}

pub fn get(config: &MutateConfig) -> Result<Box<dyn Mutate>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new(config.rate)) as Box<dyn Mutate>),
        "directed" => Ok(
            Box::new(Directed::new(config.rate, config.speed.unwrap_or(0.1))) as Box<dyn Mutate>,
        ),
        "adaptive" => Ok(Box::new(Adaptive::new(config.rate)) as Box<dyn Mutate>),
        _ => Err(anyhow::anyhow!("unknown mutate method name")),
    }
}
