use crate::population::Individual;
use crate::utils::{Assets, Grid};

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(population_size: usize, individual_size: usize, assets: &Assets) -> Self {
        Self {
            individuals: (0..population_size)
                .map(|id| Individual::new(id, individual_size, assets))
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
        self.individuals.iter().for_each(|i| {
            i.save(epoch, assets, grid);
        });
    }
}
