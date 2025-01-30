use rand::Rng;

use crate::utils::Asset;

pub struct Assets {
    pub size: usize,
    pub values: Vec<Asset>,
}

impl Assets {
    pub fn new(size: usize) -> Self {
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
        let values = colors.iter().map(|x| Asset::new(x, size)).collect();
        Self { size, values }
    }

    pub fn rand(&self) -> usize {
        rand::rng().random_range(0..self.values.len())
    }
}
