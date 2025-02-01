mod algorithm;
mod config;
mod population;
mod utils;

use anyhow::Result;
use simple_logger::SimpleLogger;

use crate::algorithm::Algorithm;
use crate::config::Config;
use crate::population::Population;
use crate::utils::{Assets, Grid};

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let config = Config::new("Config.json")?;

    log::info!("loading assets");
    let assets = Assets::new(&config.assets);
    let grid = Grid::new(config.population.individual_size);

    log::info!("loading algorithm");
    let algorithm = Algorithm::new(&config.algorithm)?;
    log::info!("creating results directory");
    std::fs::create_dir_all("results")?;

    log::info!("creating population");
    let population = Population::new(&config.population, &assets)?;

    log::info!("running algorithm");
    let (_, epoch) = algorithm.run(
        &grid,
        &assets,
        config.population.population_size,
        &population,
    )?;

    std::fs::copy(format!("results/result-{}.png", epoch), "result.png")?;
    utils::create_gif(epoch)?;
    std::fs::remove_dir_all("results")?;

    Ok(())
}
