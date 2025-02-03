use std::collections::HashMap;

use anyhow::Result;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::algorithm::Fitness;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub struct StrictUniformity;

impl Fitness for StrictUniformity {
    fn individual(&self, individual: &mut Individual, _: &Assets, _: &Grid) -> Result<()> {
        let mut colors = HashMap::with_capacity(individual.genes.len());
        for g in individual.iter() {
            *colors.entry(g.color).or_insert(0) += 1;
        }
        let max_color = (*colors.values().max().unwrap()) as f32;
        let uniformity = max_color / individual.genes.len() as f32;
        individual.fitness = uniformity;
        Ok(())
    }

    fn population(&self, population: &mut Population, assets: &Assets, grid: &Grid) -> Result<()> {
        population
            .individuals
            .par_iter_mut()
            .map(|i| self.individual(i, assets, grid))
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }
}
