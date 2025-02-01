use serde::Deserialize;

#[derive(Deserialize)]
pub struct PopulationConfig {
    pub population_size: usize,
    pub individual_size: usize,
}
