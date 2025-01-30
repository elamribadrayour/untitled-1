mod algorithm;
mod population;
mod utils;

use anyhow::Result;
use simple_logger::SimpleLogger;

use crate::algorithm::Algorithm;
use crate::population::Population;
use crate::utils::{Assets, Grid};

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let epochs = 200;
    let asset_size = 100;
    let population_size = 100;
    let individual_size = 15 * 15;

    let mutation_rate = 0.1;
    let crossover_rate = 0.5;
    let selection_rate = 0.5;

    let selection_function = "best";
    let mutation_function = "random";
    let crossover_function = "random";
    let fitness_function = "uniformity";

    log::info!(
        "running untitled-1: epochs: {}, population_size: {}, individual_size: {}",
        epochs,
        population_size,
        individual_size
    );

    log::info!("loading assets");
    let assets = Assets::new(asset_size);
    let grid = Grid::new(individual_size);

    log::info!("loading algorithm");
    let algorithm = Algorithm::new(
        fitness_function,
        selection_function,
        crossover_rate,
        crossover_function,
        mutation_rate,
        mutation_function,
        selection_rate,
    )?;

    log::info!("creating results directory");
    std::fs::create_dir_all("results")?;

    log::info!("creating population");
    let population = Population::new(population_size, individual_size, &assets);

    log::info!("running algorithm");
    algorithm.run(&grid, epochs, &assets, population_size, &population)?;

    Ok(())
}
