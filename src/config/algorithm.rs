use crate::config::{CrossoverConfig, FitnessConfig, MutateConfig, SelectConfig};
pub struct AlgorithmConfig {
    pub epochs: usize,
    pub threshold: f32,
    pub mutation_config: MutateConfig,
    pub fitness_config: FitnessConfig,
    pub selection_config: SelectConfig,
    pub crossover_config: CrossoverConfig,
}
