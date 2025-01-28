use anyhow::Result;

use crate::algorithm::select::Best;
use crate::population::Population;

pub trait Select {
    fn population(&self, population: &Population, fitnesses: &[f32]) -> Result<Population>;
}

pub fn get(name: &str, rate: f32) -> Result<Box<dyn Select>> {
    match name {
        "best" => Ok(Box::new(Best::new(rate)) as Box<dyn Select>),
        _ => Err(anyhow::anyhow!("unknown select method name")),
    }
}
