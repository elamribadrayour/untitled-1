use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AssetConfig {
    pub size: usize,
    pub image: String,
    pub nb_colors: usize,
}
