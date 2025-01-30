use anyhow::Result;

use crate::algorithm::{crossover, fitness, mutate, select};
use crate::population::Population;
use crate::utils::{Assets, Grid};

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

    pub fn fitness(
        &self,
        population: &Population,
        assets: &Assets,
        grid: &Grid,
    ) -> Result<Vec<f32>> {
        self.fitness.population(population, assets, grid)
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

    pub fn run(
        &self,
        grid: &Grid,
        epochs: usize,
        assets: &Assets,
        population_size: usize,
        population: &Population,
    ) -> Result<Population> {
        let mut fitness = 0.0;
        let mut output = Population::individuals(&population.individuals);
        for epoch in 0..epochs {
            log::info!(
                "batch: {} -- population size: {:?} -- fitness: {:?}",
                epoch,
                population.len(),
                fitness
            );
            let fitnesses = self.fitness(&output, assets, grid)?;
            fitness = *fitnesses
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            output = self.select(&output, &fitnesses)?;
            output = self.crossover(&output, population_size)?;
            output = self.mutate(&output, assets)?;

            if epoch % 100 == 0 {
                output.save(epoch, assets, grid);
            }
        }
        Ok(output)
    }
}
