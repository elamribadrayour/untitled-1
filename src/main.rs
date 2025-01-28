mod algorithm;
mod population;
mod utils;

use anyhow::Result;

use crate::algorithm::Algorithm;
use crate::population::Population;
use crate::utils::Assets;

fn main() -> Result<()> {
    let epochs = 20;
    let population_size = 3;
    let individual_size = 10 * 10;

    let assets = Assets::new();
    let algorithm = Algorithm::new("random", "best", 0.8, "random", 0.5, "random", 0.5)?;

    std::fs::create_dir_all("results")?;

    let mut population = Population::new(population_size, individual_size, &assets);
    for batch in 0..epochs {
        println!(
            "batch: {} -- population size: {:?}",
            batch,
            population.len()
        );
        let fitnesses = algorithm.fitness(&population)?;
        population = algorithm.select(&population, &fitnesses)?;
        population = algorithm.crossover(&population, population_size)?;
        population = algorithm.mutate(&population, &assets)?;
        population.save(batch)
    }

    Ok(())
}
