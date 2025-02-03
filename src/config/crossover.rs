use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CrossoverConfig {
    pub name: String,
    pub rate: Option<f32>,
}
