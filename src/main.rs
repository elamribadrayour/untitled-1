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
    let data_dir = format!(
        "results/exec_date={}",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    );
    std::fs::create_dir_all(&data_dir)?;

    log::info!("loading assets");
    let assets = Assets::new(&config.assets)?;
    let grid = Grid::new(config.population.individual_size);

    log::info!("loading algorithm");
    let mut algorithm = Algorithm::new(&config.algorithm)?;

    log::info!("creating population");
    let mut population = Population::new(&config.population, &assets)?;

    log::info!("running algorithm");
    let (_, epoch) = algorithm.run(
        &grid,
        &assets,
        config.population.population_size,
        &mut population,
        &data_dir,
    )?;

    utils::create_gif(epoch, &data_dir)?;
    config.save(&data_dir)?;
    Ok(())
}
