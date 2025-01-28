use anyhow::Result;

use crate::algorithm::fitness::Random;
use crate::population::Population;

pub trait Fitness {
    fn population(&self, population: &Population) -> Result<Vec<f32>>;
}

pub fn get(name: &str) -> Result<Box<dyn Fitness>> {
    match name {
        "random" => Ok(Box::new(Random::new()) as Box<dyn Fitness>),
        _ => Err(anyhow::anyhow!("unknown fitness method name")),
    }
}
