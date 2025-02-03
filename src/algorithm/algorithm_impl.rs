use std::time::Instant;

use anyhow::Result;

use crate::algorithm::{crossover, fitness, mutate, select};
use crate::config::AlgorithmConfig;
use crate::population::Population;
use crate::utils::{Assets, Grid, Plot};

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

    pub fn fitness(&self, population: &mut Population, assets: &Assets, grid: &Grid) -> Result<()> {
        self.fitness.population(population, assets, grid)?;
        Ok(())
    }

    pub fn get_max(&self, population: &Population) -> (usize, f32) {
        let max = population
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
            .fitness;

        (
            population
                .iter()
                .position(|i| (i.fitness - max).abs() < f32::EPSILON)
                .unwrap(),
            max,
        )
    }

    pub fn mutate(&self, population: &Population, assets: &Assets) -> Result<Population> {
        self.mutate.population(population, assets)
    }

    pub fn select(&self, population: &Population) -> Result<Population> {
        self.select.population(population)
    }

    pub fn crossover(&self, population: &Population, size: usize) -> Result<Population> {
        self.crossover.population(population, size)
    }

    pub fn run(
        &mut self,
        grid: &Grid,
        assets: &Assets,
        population_size: usize,
        population: &mut Population,
        data_dir: &str,
    ) -> Result<(Population, usize)> {
        let mut max_fitnesses: Vec<f32> = Vec::new();
        let mut output = Population::individuals(population.individuals.clone());

        let mut epoch = 0;
        while epoch < self.epochs {
            let start = Instant::now();

            self.fitness(&mut output, assets, grid)?; // 180 ms

            let (index, max) = self.get_max(&output);
            max_fitnesses.push(max);

            output = self.select(&output)?; // 266ms
            output = self.crossover(&output, population_size)?; // 100ms

            output = self.mutate(&output, assets)?;
            self.mutate.update(max);

            if epoch % 10 == 0 {
                log::info!(
                    "epoch: {} -- fitness: {:?} -- mutate rate: {:?} -- time: {:?}ms",
                    epoch,
                    max_fitnesses.last().unwrap(),
                    self.mutate.current_rate(),
                    start.elapsed().as_millis(),
                );
            }

            output.save_best(epoch, assets, grid, index, data_dir)?;
            if (max_fitnesses.last().unwrap() - self.threshold).abs() < f32::EPSILON {
                log::info!(
                    "early stopping: fitness is greater than threshold {}",
                    self.threshold
                );
                break;
            }

            epoch += 1;
        }
        Plot::fitness(&max_fitnesses, epoch, data_dir)?;
        Ok((output, epoch))
    }
}
