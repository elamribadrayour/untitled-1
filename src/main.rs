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
        population_size: 100,
        individual_size: 15 * 15,
    };
    let algorithm_config = AlgorithmConfig {
        epochs: 1000,
        threshold: 1.0,
        mutation_config: MutateConfig {
            name: "random".to_string(),
            rate: 0.01,
        },
        crossover_config: CrossoverConfig {
            name: "random".to_string(),
            rate: 0.3,
        },
        selection_config: SelectConfig {
            name: "best".to_string(),
            rate: 0.3,
        },
        fitness_config: FitnessConfig {
            name: "strict-uniformity".to_string(),
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
    let (_, epoch) = algorithm.run(
        &grid,
        &assets,
        population_config.population_size,
        &population,
    )?;

    std::fs::copy(format!("results/result-{}.png", epoch), "result.png")?;
    utils::create_gif(epoch)?;
    std::fs::remove_dir_all("results")?;

    Ok(())
}
