use anyhow::Result;
use rand::seq::IndexedRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::algorithm::Select;
use crate::population::{Individual, Population};

pub struct Proportionate {
    rate: f32,
}

impl Proportionate {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl Select for Proportionate {
    fn population(&self, population: &Population) -> Result<Population> {
        let input_size = population.len();
        let total = population.iter().map(|i| i.fitness).sum::<f32>();
        let output_size = (self.rate * input_size as f32) as usize;
        let individuals = (0..output_size)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| {
                let mut cumulative = 0.0;
                let mut ids = (0..input_size).collect::<Vec<usize>>();
                for _ in 0..input_size {
                    let id = ids.choose(rng).unwrap();
                    cumulative += population.individuals[*id].fitness / total;
                    if cumulative > 1.0 {
                        return population.individuals[*id].clone();
                    }
                    ids = ids
                        .iter()
                        .cloned()
                        .filter(|x| *x != *id)
                        .collect::<Vec<usize>>();
                }
                population.individuals.last().unwrap().clone()
            })
            .collect::<Vec<Individual>>();

        Ok(Population::individuals(individuals))
    }
}
