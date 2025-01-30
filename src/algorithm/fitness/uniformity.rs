use std::collections::HashSet;

use anyhow::Result;

use crate::algorithm::Fitness;
use crate::population::{Gene, Individual, Population};
use crate::utils::{Assets, Grid};

pub struct Uniformity {
    range: f32,
}

impl Uniformity {
    pub fn new(range: f32) -> Self {
        Self { range }
    }
}

impl Fitness for Uniformity {
    fn gene(&self, gene: &Gene, individual: &Individual, _: &Assets, grid: &Grid) -> Result<f32> {
        let neighbors = grid.neighbors(gene.id, self.range as usize);
        let nb_unique_colors = neighbors
            .iter()
            .map(|id| individual.genes[*id].color)
            .collect::<HashSet<_>>()
            .len() as f32;
        let uniformity = 1.0 - nb_unique_colors / neighbors.len() as f32;
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
