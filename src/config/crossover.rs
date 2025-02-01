use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrossoverConfig {
    pub name: String,
    pub rate: f32,
}
