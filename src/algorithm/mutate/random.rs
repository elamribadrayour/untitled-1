use anyhow::Result;
use rand::Rng;

use crate::algorithm::Mutate;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub struct Random {
    rate: f32,
}

impl Random {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Mutate for Random {
    fn gene(&self, gene: &Gene, assets: &Assets) -> Result<Gene> {
        if rand::rng().random_range(0.0..1.0) < self.rate {
            Ok(Gene::new(assets))
        } else {
            Ok(gene.clone())
        }
    }

    fn individual(&self, individual: &Individual, assets: &Assets) -> Result<Individual> {
        let id = individual.id;
        let genes = individual
            .genes
            .iter()
            .map(|gene| self.gene(gene, assets).unwrap())
            .collect::<Vec<Gene>>();
        Ok(Individual::genes(&genes, id))
    }

    fn population(&self, population: &Population, assets: &Assets) -> Result<Population> {
        let individuals = population
            .individuals
            .iter()
            .map(|i| self.individual(i, assets).unwrap())
            .collect::<Vec<Individual>>();
        Ok(Population::individuals(&individuals))
    }
}
