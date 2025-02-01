use anyhow::Result;
use rand::seq::IndexedRandom;

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
    fn population(&self, population: &Population, fitnesses: &[f32]) -> Result<Population> {
        let mut rng = rand::rng();

        let input_size = population.len();
        let total = fitnesses.iter().sum::<f32>();
        let output_size = (self.rate * input_size as f32) as usize;
        let individuals = (0..output_size)
            .map(|_| {
                let mut cumulative = 0.0;
                let mut ids = (0..input_size).collect::<Vec<usize>>();
                for i in 0..input_size {
                    let id = ids.choose(&mut rng).unwrap();
                    cumulative += fitnesses[*id] / total;
                    if cumulative > 1.0 {
                        return population.individuals[i].clone();
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
