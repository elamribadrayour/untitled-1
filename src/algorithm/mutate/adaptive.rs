use anyhow::Result;
use rand::distr::{Distribution, Uniform};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::algorithm::Mutate;
use crate::population::{Gene, Individual, Population};
use crate::utils::Assets;

pub struct Adaptive {
    max_rate: f32,
    current_rate: f32,
    fitnesses: Vec<f32>,
}

impl Adaptive {
    pub fn new(rate: f32) -> Self {
        Self {
            max_rate: rate,
            current_rate: rate,
            fitnesses: vec![],
        }
    }
}

impl Mutate for Adaptive {
    fn update(&mut self, fitness: f32) {
        self.fitnesses.push(fitness);
        // We calculate the mean value of 3 fitnesses before the current epoch
        let mean = self.fitnesses.iter().rev().skip(1).take(3).sum::<f32>() / 3.0;
        if (fitness - mean) < 0.001 && self.current_rate < self.max_rate {
            // Fitness is not evolving
            // We need to increase the mutation rate to avoid stagnation
            self.current_rate = (self.current_rate * 1.1).clamp(0.0, self.max_rate);
            log::info!(
                "fitness is not evolving, increasing mutation rate to {}",
                self.current_rate
            );
        } else if (fitness - mean) > 0.1 {
            // Fitness is evolving
            // We need to reduce the mutation rate to avoid overfitting
            self.current_rate = (self.current_rate * 0.9).clamp(0.0, self.max_rate);
            log::info!(
                "fitness is evolving, reducing mutation rate to {}",
                self.current_rate
            );
        }
    }

    fn current_rate(&self) -> f32 {
        self.current_rate
    }

    fn gene(&self, gene: &Gene, color: (f32, usize)) -> Result<Gene> {
        if color.0 < self.current_rate {
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
        let range_uniform = Uniform::new(0.0, 1.0)?;
        let color_uniform = Uniform::new(-(assets.len() as i32) as f32, assets.len() as f32)?;

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
