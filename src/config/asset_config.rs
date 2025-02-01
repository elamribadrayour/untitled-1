use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetConfig {
    pub size: usize,
    pub image: String,
    pub nb_colors: usize,
}
