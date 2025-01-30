use anyhow::Result;

use crate::algorithm::select::Best;
use crate::config::SelectConfig;
use crate::population::Population;

pub trait Select {
    fn population(&self, population: &Population, fitnesses: &[f32]) -> Result<Population>;
}

pub fn get(config: &SelectConfig) -> Result<Box<dyn Select>> {
    match config.name.as_str() {
        "best" => Ok(Box::new(Best::new(config.rate)) as Box<dyn Select>),
        _ => Err(anyhow::anyhow!("unknown select method name")),
    }
}
