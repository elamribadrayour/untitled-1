use anyhow::Result;

use crate::algorithm::{crossover, fitness, mutate, select};
use crate::config::AlgorithmConfig;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub struct Algorithm {
    pub epochs: usize,
    pub mutate: Box<dyn mutate::Mutate>,
    pub select: Box<dyn select::Select>,
    pub fitness: Box<dyn fitness::Fitness>,
    pub crossover: Box<dyn crossover::Crossover>,
}

impl Algorithm {
    pub fn new(config: &AlgorithmConfig) -> Result<Self> {
        Ok(Self {
            epochs: config.epochs,
            fitness: fitness::get(&config.fitness_config)?,
            mutate: mutate::get(&config.mutation_config)?,
            select: select::get(&config.selection_config)?,
            crossover: crossover::get(&config.crossover_config)?,
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

    pub fn sort(&self, population: &Population, fitnesses: &[f32]) -> Result<Population> {
        let mut individuals = population
            .individuals
            .iter()
            .zip(fitnesses)
            .collect::<Vec<(&Individual, &f32)>>();
        individuals
            .sort_by(|(_, &f1), (_, &f2)| f2.partial_cmp(&f1).unwrap_or(std::cmp::Ordering::Equal));
        let individuals = individuals
            .iter()
            .map(|(i, _)| (*i).clone())
            .collect::<Vec<Individual>>();
        Ok(Population::individuals(&individuals))
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
        assets: &Assets,
        population_size: usize,
        population: &Population,
    ) -> Result<Population> {
        let mut fitness = 0.0;
        let mut output = Population::individuals(&population.individuals);
        for epoch in 0..self.epochs {
            log::info!(
                "batch: {} -- population size: {:?} -- fitness: {:?}",
                epoch,
                population.len(),
                fitness
            );
            let fitnesses = self.fitness(&output, assets, grid)?;
            output = self.sort(&output, &fitnesses)?;
            fitness = *fitnesses
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            output.save(epoch, assets, grid);

            output = self.select(&output, &fitnesses)?;
            output = self.crossover(&output, population_size)?;
            output = self.mutate(&output, assets)?;
        }
        Ok(output)
    }
}
