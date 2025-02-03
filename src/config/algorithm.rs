use serde::{Deserialize, Serialize};

use crate::config::{CrossoverConfig, FitnessConfig, MutateConfig, SelectConfig};

#[derive(Deserialize, Serialize)]
pub struct AlgorithmConfig {
    pub epochs: usize,
    pub threshold: f32,
    pub mutation: MutateConfig,
    pub fitness: FitnessConfig,
    pub selection: SelectConfig,
    pub crossover: CrossoverConfig,
}
