use std::collections::HashMap;

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::algorithm::Fitness;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub struct StrictUniformity;

impl Fitness for StrictUniformity {
    fn individual(&self, individual: &Individual, _: &Assets, _: &Grid) -> Result<f32> {
        let mut colors = HashMap::with_capacity(individual.genes.len());
        for g in individual.iter() {
            *colors.entry(g.color).or_insert(0) += 1;
        }
        let max_color = (*colors.values().max().unwrap()) as f32;
        let uniformity = max_color / individual.genes.len() as f32;
        Ok(uniformity)
    }

    fn population(
        &self,
        population: &Population,
        assets: &Assets,
        grid: &Grid,
    ) -> Result<Vec<f32>> {
        let fitnesses = population
            .individuals
            .par_iter()
            .map(|i| self.individual(i, assets, grid))
            .collect::<Result<Vec<_>>>()?;
        Ok(fitnesses)
    }
}
