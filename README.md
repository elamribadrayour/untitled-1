# Untitled-1

This project is a straightforward implementation of a genetic algorithm, designed for anyone curious to understand the steps involved in such algorithms. The assets are genes of different colors generated to simulate the genetic process. The goal is to help users understand the evolutionary process that each individual undergoes during a genetic algorithm evolution, as visualized through the images saved.

<div align="center">
    <table>
        <tr>
            <td><img src="result-1.gif" width="500" height="500" /></td>
            <td><img src="result-2.gif" width="500" height="500" /></td>
        </tr>
    </table>
</div>

## Project Structure

- **src/main.rs**: The entry point of the application. It initializes the algorithm and population, and runs the genetic algorithm for a specified number of epochs.
- **src/algorithm/**: Contains the implementation of the genetic algorithm, including modules for crossover, mutation, selection, and fitness evaluation.
- **src/population/**: Contains the implementation of the population, individuals, and genes.
- **src/utils/**: Contains utility modules for handling assets and images.

## Dependencies

The project uses the following dependencies, as specified in `Cargo.toml`:

- `gif`: For generating GIFs.
- `anyhow`: For error handling.
- `simple_logger`: For logging.
- `image`: For image processing.
- `chrono`: For logging the time.
- `rayon`: For parallel processing.
- `serde_json`: For parsing JSON data.
- `rand`: For random number generation.
- `plotters`: For plotting the results.
- `hex_color`: For parsing hexadecimal color codes.
- `serde`: For serializing and deserializing JSON data.

## Building and Running

To build and run the project, ensure you have Git, Rust and Cargo installed. Then, execute the following commands:

```
git clone https://github.com/elamribadrayour/untitled-1.git
cd untitled-1
cargo run --release
```

## Usage

The application runs a genetic algorithm over a specified number of epochs. It prints the batch number and population size at each epoch. The algorithm involves the following steps:

1. **Fitness Evaluation**: Evaluates the fitness of the population.
2. **Selection**: Selects individuals based on their fitness.
3. **Crossover**: Combines individuals to create new offspring.
4. **Mutation**: Mutates individuals to introduce variation.

## Principles


### Fitness

* **Random**: Generates a random fitness value for each population, inidividual and gene.
* **Strict Uniformity**: Checks how much the colors are uniform in an individual.

### Selection

* **Random**: Selects a random individual from the population.
* **Truncation**: Selects the best individuals from the population.
* **Proportionate**: Selects the best individuals by roulette wheel cumulation of probabilities based on fitness.

### Crossover

* **Random**: Selects a random gene from the parents and copies it to the offspring.

### Mutation

* **Random**: Change the color a gene randomly.
* **Directed**: Change the color of a gene by a range from the previous color.
* **Adaptive**: Change the color of a gene by a range from the previous color but adapt the rate of mutation based on the fitness evolution.

## License

This project is licensed under the WTFPL License. See the [LICENSE](LICENSE) file for details.