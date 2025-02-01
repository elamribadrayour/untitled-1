use anyhow::Result;
use image::imageops::FilterType;
use rand::distr::{Distribution, Uniform};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::config::AssetConfig;
use crate::utils::Asset;

pub struct Assets {
    pub size: usize,
    pub values: Vec<Asset>,
}

impl Assets {
    pub fn new(config: &AssetConfig) -> Self {
        let colors = [
            // Reds
            "#ff0000", // Pure Red
            "#ff2b2b", // Bright Red
            "#d64161", // Rose Red
            "#ff6b6b", // Light Red
            // Oranges
            "#ff6f61", // Coral
            "#ff8c42", // Dark Orange
            "#ffa07a", // Light Salmon
            // Yellows
            "#feb236", // Golden Yellow
            "#fff700", // Bright Yellow
            "#f9dc5c", // Muted Yellow
            // Greens
            "#84bc9c", // Light Green
            "#2ca58d", // Teal
            "#228b22", // Forest Green
            "#32cd32", // Lime Green
            // Blues
            "#0a2342", // Dark Blue
            "#4169e1", // Royal Blue
            "#00bfff", // Deep Sky Blue
            "#87ceeb", // Sky Blue
            // Purples
            "#6b5b95", // Purple
            "#9932cc", // Dark Orchid
            "#da70d6", // Orchid
            "#e6e6fa", // Lavender
            // Neutral/Special
            "#fffdf7", // Off-white
            "#6b4226", // Brown
            "#808080", // Gray
        ];
        let image = match config.image.as_str() {
            "empty" => image::ImageBuffer::from_pixel(
                config.size as u32,
                config.size as u32,
                image::Rgba([0, 0, 0, 255]), // Black pixel
            ),
            x => image::open(format!("assets/{}.png", x))
                .unwrap()
                .resize(config.size as u32, config.size as u32, FilterType::Lanczos3)
                .to_rgba8(),
        };

        let values: Vec<Asset> = colors
            .iter()
            .take(config.nb_colors)
            .map(|x| Asset::new(x, &image))
            .collect();
        Self {
            size: config.size,
            values,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn choose(&self, size: usize) -> Result<Vec<usize>> {
        let len = self.values.len();
        let range = Uniform::new(0, len)?;
        let colors = (0..size)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| range.sample(rng))
            .collect::<Vec<usize>>();
        Ok(colors)
    }
}
