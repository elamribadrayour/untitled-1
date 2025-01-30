mod algorithm;
mod config;
mod population;
mod utils;

use anyhow::Result;
use simple_logger::SimpleLogger;

use crate::algorithm::Algorithm;
use crate::config::{
    AlgorithmConfig, CrossoverConfig, FitnessConfig, MutateConfig, PopulationConfig, SelectConfig,
};
use crate::population::Population;
use crate::utils::{Assets, Grid};

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let population_config = PopulationConfig {
        nb_colors: 100,
        population_size: 10,
        individual_size: 15 * 15,
    };
    let algorithm_config = AlgorithmConfig {
        epochs: 100,
        mutation_config: MutateConfig {
            name: "random".to_string(),
            rate: 0.1,
        },
        crossover_config: CrossoverConfig {
            name: "random".to_string(),
            rate: 0.5,
        },
        selection_config: SelectConfig {
            name: "best".to_string(),
            rate: 0.5,
        },
        fitness_config: FitnessConfig {
            name: "uniformity".to_string(),
            range: 2.0,
        },
    };

    log::info!("loading assets");
    let assets = Assets::new(population_config.nb_colors);
    let grid = Grid::new(population_config.individual_size);

    log::info!("loading algorithm");
    let algorithm = Algorithm::new(&algorithm_config)?;
    log::info!("creating results directory");
    std::fs::create_dir_all("results")?;

    log::info!("creating population");
    let population = Population::new(&population_config, &assets);

    log::info!("running algorithm");
    algorithm.run(
        &grid,
        &assets,
        population_config.population_size,
        &population,
    )?;
    utils::create_gif(algorithm_config.epochs)?;

    Ok(())
}
