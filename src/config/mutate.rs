use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MutateConfig {
    pub name: String,
    pub rate: f32,
    pub speed: Option<f32>,
}
