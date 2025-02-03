use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SelectConfig {
    pub name: String,
    pub rate: f32,
}
