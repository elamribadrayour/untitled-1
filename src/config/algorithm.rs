use serde::Deserialize;

use crate::config::{CrossoverConfig, FitnessConfig, MutateConfig, SelectConfig};

#[derive(Deserialize)]
pub struct AlgorithmConfig {
    pub epochs: usize,
    pub threshold: f32,
    pub mutation: MutateConfig,
    pub fitness: FitnessConfig,
    pub selection: SelectConfig,
    pub crossover: CrossoverConfig,
}
