use serde::Deserialize;

#[derive(Deserialize)]
pub struct MutateConfig {
    pub name: String,
    pub rate: f32,
    pub speed: Option<f32>,
}
