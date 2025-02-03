use anyhow::Result;

use crate::config::PopulationConfig;
use crate::population::Individual;
use crate::utils::{Assets, Grid};

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(config: &PopulationConfig, assets: &Assets) -> Result<Self> {
        let individuals = (0..config.population_size)
            .map(|id| Individual::new(id, &assets.choose(config.individual_size)?))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { individuals })
    }

    pub fn individuals(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }

    pub fn save_best(
        &self,
        epoch: usize,
        assets: &Assets,
        grid: &Grid,
        index: usize,
        data_dir: &str,
    ) -> Result<()> {
        let best_dir = format!("{}/type=best/epoch={}", data_dir, epoch);
        std::fs::create_dir_all(&best_dir)?;
        self.individuals[index].save(assets, grid, &best_dir)?;
        std::fs::rename(
            format!("{}/{}.png", best_dir, self.individuals[index].id),
            format!("{}/best.png", best_dir),
        )?;
        Ok(())
    }
}
