use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use serde::Deserialize;

use crate::config::AlgorithmConfig;
use crate::config::AssetConfig;
use crate::config::PopulationConfig;

#[derive(Deserialize)]
pub struct Config {
    pub algorithm: AlgorithmConfig,
    pub population: PopulationConfig,
    pub assets: AssetConfig,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
