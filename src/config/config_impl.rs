use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::AlgorithmConfig;
use crate::config::AssetConfig;
use crate::config::PopulationConfig;

#[derive(Deserialize, Serialize)]
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
        if config.algorithm.epochs < 1 {
            return Err(anyhow::anyhow!("epochs must be at least 1"));
        }
        if config.assets.nb_colors < 2 {
            return Err(anyhow::anyhow!("nb_colors must be at least 1"));
        }
        Ok(config)
    }

    pub fn save(&self, data_dir: &str) -> Result<()> {
        let config_path = format!("{}/config.json", data_dir);
        let file = File::create(config_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
