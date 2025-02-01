use anyhow::Result;
use rand::distr::{Distribution, Uniform};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    fn gene(&self, gene: &Gene, color: (f32, usize)) -> Result<Gene> {
        if color.0 < self.rate {
            Ok(Gene::new(gene.id, color.1))
        } else {
            Ok(gene.clone())
        }
    }

    fn individual(&self, individual: &Individual, colors: &[(f32, usize)]) -> Result<Individual> {
        let id = individual.id;
        let genes = (0..individual.len())
            .map(|i| self.gene(&individual.genes[i], colors[i]).unwrap())
            .collect::<Vec<Gene>>();
        Ok(Individual::genes(&genes, id))
    }

    fn population(&self, population: &Population, assets: &Assets) -> Result<Population> {
        if population.is_empty() {
            return Err(anyhow::anyhow!("population is empty"));
        }

        let population_size = population.len();
        let individual_size = population.individuals[0].len();

        let range_uniform = Uniform::new(0.0, 1.0)?;
        let color_uniform = Uniform::new(0, assets.len())?;

        let colors = (0..population_size)
            .into_par_iter()
            .map(|_| {
                (0..individual_size)
                    .into_par_iter()
                    .map_init(rand::rng, |rng, _| {
                        (range_uniform.sample(rng), color_uniform.sample(rng))
                    })
                    .collect::<Vec<(f32, usize)>>()
            })
            .collect::<Vec<Vec<(f32, usize)>>>();

        let individuals = (0..population_size)
            .into_par_iter()
            .map(|id| self.individual(&population.individuals[id], &colors[id]))
            .collect::<Result<Vec<Individual>>>()?;

        let population = Population::individuals(individuals);
        Ok(population)
    }
}
