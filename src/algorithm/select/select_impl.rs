use anyhow::Result;

use crate::algorithm::select::{Proportionate, Random, Truncation};
use crate::config::SelectConfig;
use crate::population::Population;

pub trait Select {
    fn population(&self, population: &Population) -> Result<Population>;
}

pub fn get(config: &SelectConfig) -> Result<Box<dyn Select>> {
    match config.name.as_str() {
        "random" => Ok(Box::new(Random::new()) as Box<dyn Select>),
        "truncation" => Ok(Box::new(Truncation::new(config.rate)) as Box<dyn Select>),
        "proportionate" => Ok(Box::new(Proportionate::new(config.rate)) as Box<dyn Select>),
        _ => Err(anyhow::anyhow!("unknown select method name")),
    }
}
