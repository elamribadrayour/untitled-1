use anyhow::Result;
use rand::distr::{Distribution, Uniform};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::algorithm::Mutate;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub struct Directed {
    rate: f32,
    speed: f32,
}

impl Directed {
    pub fn new(rate: f32, speed: f32) -> Self {
        Self { rate, speed }
    }
}

impl Mutate for Directed {
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

        let nb_colors = assets.len();
        let population_size = population.len();
        let speed = (self.speed * nb_colors as f32) as i32;
        let range_uniform = Uniform::new(0.0, 1.0)?;
        let color_uniform = Uniform::new(-speed, speed)?;

        let individuals = (0..population_size)
            .into_par_iter()
            .map(|id| {
                let individual = &population.individuals[id];
                let colors = individual
                    .genes
                    .par_iter()
                    .map_init(rand::rng, |rng, gene| {
                        let range = range_uniform.sample(rng);
                        let speed = color_uniform.sample(rng);
                        let color = (gene.color + speed as usize).max(0).min(nb_colors - 1);
                        (range, color)
                    })
                    .collect::<Vec<(f32, usize)>>();
                self.individual(individual, &colors)
            })
            .collect::<Result<Vec<Individual>>>()?;

        let population = Population::individuals(individuals);
        Ok(population)
    }
}
