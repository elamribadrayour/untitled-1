use serde::Deserialize;

#[derive(Deserialize)]
pub struct SelectConfig {
    pub name: String,
    pub rate: f32,
}
