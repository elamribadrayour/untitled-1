use crate::config::{CrossoverConfig, FitnessConfig, MutateConfig, SelectConfig};
pub struct AlgorithmConfig {
    pub epochs: usize,
    pub mutation_config: MutateConfig,
    pub crossover_config: CrossoverConfig,
    pub selection_config: SelectConfig,
    pub fitness_config: FitnessConfig,
}
