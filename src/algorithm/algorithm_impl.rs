use std::time::Instant;

use anyhow::Result;

use crate::algorithm::{crossover, fitness, mutate, select};
use crate::config::AlgorithmConfig;
use crate::population::{Individual, Population};
use crate::utils::{Assets, Grid};

pub struct Algorithm {
    pub epochs: usize,
    pub threshold: f32,
    pub mutate: Box<dyn mutate::Mutate>,
    pub select: Box<dyn select::Select>,
    pub fitness: Box<dyn fitness::Fitness>,
    pub crossover: Box<dyn crossover::Crossover>,
}

impl Algorithm {
    pub fn new(config: &AlgorithmConfig) -> Result<Self> {
        Ok(Self {
            epochs: config.epochs,
            threshold: config.threshold,
            fitness: fitness::get(&config.fitness)?,
            mutate: mutate::get(&config.mutation)?,
            select: select::get(&config.selection)?,
            crossover: crossover::get(&config.crossover)?,
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

        Ok(Population::individuals(individuals))
    }

    pub fn get_max(&self, fitnesses: &[f32]) -> f32 {
        *fitnesses
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
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
    ) -> Result<(Population, usize)> {
        let mut output = Population::individuals(population.individuals.clone());
        let mut epoch = 0;
        while epoch < self.epochs {
            epoch += 1;
            let start = Instant::now();

            let fitnesses = self.fitness(&output, assets, grid)?; // 180 ms

            output = self.sort(&output, &fitnesses)?; //455 ms
            output.save(epoch, assets, grid);

            let fitness = self.get_max(&fitnesses);

            if (fitness - self.threshold).abs() < f32::EPSILON {
                log::info!(
                    "early stopping: fitness is greater than threshold {}",
                    self.threshold
                );
                break;
            }

            output = self.select(&output, &fitnesses)?; // 266ms
            output = self.crossover(&output, population_size)?; // 100ms
            output = self.mutate(&output, assets)?;

            if epoch % 100 == 0 {
                log::info!(
                    "epoch: {} -- fitness: {:?} -- time: {:?}ms",
                    epoch,
                    fitness,
                    start.elapsed().as_millis()
                );
            }
        }
        Ok((output, epoch))
    }
}
