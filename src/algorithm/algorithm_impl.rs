use anyhow::Result;

use crate::algorithm::{crossover, fitness, mutate, select};
use crate::population::Population;
use crate::utils::Assets;

pub struct Algorithm {
    pub mutate: Box<dyn mutate::Mutate>,
    pub select: Box<dyn select::Select>,
    pub fitness: Box<dyn fitness::Fitness>,
    pub crossover: Box<dyn crossover::Crossover>,
}

impl Algorithm {
    pub fn new(
        fitness: &str,
        select: &str,
        select_rate: f32,
        crossover: &str,
        crossover_rate: f32,
        mutate: &str,
        mutate_rate: f32,
    ) -> Result<Self> {
        Ok(Self {
            fitness: fitness::get(fitness)?,
            mutate: mutate::get(mutate, mutate_rate)?,
            select: select::get(select, select_rate)?,
            crossover: crossover::get(crossover, crossover_rate)?,
        })
    }

    pub fn fitness(&self, population: &Population) -> Result<Vec<f32>> {
        self.fitness.population(population)
    }

    pub fn mutate(&self, population: &Population, assets: &Assets) -> Result<Population> {
        self.mutate.population(population, assets)
    }

    pub fn select(&self, population: &Population, fitnesses: &[f32]) -> Result<Population> {
        self.select.population(population, fitnesses)
    }

    pub fn crossover(&self, population: &Population, size: usize) -> Result<Population> {
        self.crossover.population(population, size)
    }
}
