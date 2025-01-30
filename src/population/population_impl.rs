use crate::config::PopulationConfig;
use crate::population::Individual;
use crate::utils::{Assets, Grid};

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(config: &PopulationConfig, assets: &Assets) -> Self {
        Self {
            individuals: (0..config.population_size)
                .map(|id| Individual::new(id, config.individual_size, assets))
                .collect(),
        }
    }

    pub fn individuals(individuals: &[Individual]) -> Self {
        Self {
            individuals: individuals.to_vec(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn save(&self, epoch: usize, assets: &Assets, grid: &Grid) {
        self.individuals[0].save(epoch, assets, grid);
    }
}
