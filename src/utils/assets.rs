use image::ImageBuffer;
use rand::seq::IndexedRandom;

use crate::utils::Asset;

pub struct Assets {
    pub values: Vec<Asset>,
}

impl Assets {
    pub fn new() -> Self {
        let colors = [
            "#2ca58d", // Teal
            "#f46197", // Pink
            "#fffdf7", // Off-white
            "#0a2342", // Dark Blue
            "#84bc9c", // Light Green
            "#ff6f61", // Coral
            "#6b5b95", // Purple
            "#feb236", // Yellow
            "#d64161", // Red
            "#6b4226", // Brown
        ];
        let values = colors.iter().map(|x| Asset::new(x)).collect();
        Self { values }
    }

    pub fn rand(&self) -> ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
        let mut rng = rand::rng();
        self.values.choose(&mut rng).unwrap().image.clone()
    }
}
