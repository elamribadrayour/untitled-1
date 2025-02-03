use crate::algorithm::crossover::{Best, Random};
use crate::config::CrossoverConfig;
use crate::population::{Individual, Population};
use anyhow::Result;

pub trait Crossover {
    fn individual(
        &self,
        id: usize,
        p1: &Individual,
        p2: &Individual,
        rngs: &[f32],
    ) -> Result<Individual>;
    fn population(&self, population: &Population, size: usize) -> Result<Population>;
}

pub fn get(config: &CrossoverConfig) -> Result<Box<dyn Crossover>> {
    match config.name.as_str() {
        "best" => Ok(Box::new(Best) as Box<dyn Crossover>),
        "random" => Ok(Box::new(Random::new(config.rate.unwrap_or(0.5))) as Box<dyn Crossover>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
