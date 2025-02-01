use anyhow::Result;
use rand::distr::Uniform;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::algorithm::crossover::Crossover;
use crate::population::{Gene, Individual, Population};

pub struct Random {
    rate: f32,
}

impl Random {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Crossover for Random {
    fn individual(
        &self,
        id: usize,
        p1: &Individual,
        p2: &Individual,
        rngs: &[f32],
    ) -> Result<Individual> {
        if p1.is_empty() || p1.len() != p2.len() {
            return Err(anyhow::anyhow!("parents empty or lengths mismatch"));
        }

        let size = p1.len();
        let genes = (0..size)
            .map(|i| {
                if rngs[i] < self.rate {
                    p1.genes[i].clone()
                } else {
                    p2.genes[i].clone()
                }
            })
            .collect::<Vec<Gene>>();

        Ok(Individual::genes(&genes, id))
    }

    fn population(&self, population: &Population, size: usize) -> Result<Population> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("random crossover: population is empty"));
        }

        let range_distr = Uniform::new(0.0, 1.0)?;
        let population_distr = Uniform::new(0, population.len())?;

        let nb_genes = population.individuals[0].len();
        let rngs = (0..size)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| {
                (0..nb_genes)
                    .map(|_| rng.sample(range_distr))
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<Vec<f32>>>();

        let inputs = (0..size)
            .into_par_iter()
            .map_init(rand::rng, |rng, id| {
                let rngs = rngs[id].clone();
                let p1 = &population.individuals[rng.sample(population_distr)];
                let p2 = &population.individuals[rng.sample(population_distr)];
                (id, p1, p2, rngs)
            })
            .collect::<Vec<(usize, &Individual, &Individual, Vec<f32>)>>();

        let individuals = inputs
            .par_iter()
            .map(|(id, p1, p2, rngs)| self.individual(*id, p1, p2, rngs))
            .collect::<Result<Vec<Individual>>>()?;

        Ok(Population::individuals(individuals))
    }
}
