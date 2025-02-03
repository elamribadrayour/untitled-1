use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PopulationConfig {
    pub population_size: usize,
    pub individual_size: usize,
}
