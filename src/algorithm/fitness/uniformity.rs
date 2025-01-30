use std::collections::HashSet;

use anyhow::Result;

use crate::algorithm::Fitness;
use crate::population::{Gene, Individual, Population};
use crate::utils::{Assets, Grid};

pub struct Uniformity;

impl Uniformity {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fitness for Uniformity {
    fn gene(&self, gene: &Gene, individual: &Individual, _: &Assets, grid: &Grid) -> Result<f32> {
        let uniformity = grid
            .neighbors(gene.id, 10)
            .iter()
            .map(|id| individual.genes[*id].color)
            .collect::<HashSet<_>>()
            .len() as f32
            / 5.0;
        Ok(uniformity)
    }

    fn individual(&self, individual: &Individual, assets: &Assets, grid: &Grid) -> Result<f32> {
        let uniformity = individual
            .iter()
            .map(|g| self.gene(g, individual, assets, grid))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum::<f32>()
            / individual.len() as f32;
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
            .iter()
            .map(|i| self.individual(i, assets, grid))
            .collect::<Result<Vec<_>>>()?;
        Ok(fitnesses)
    }
}
