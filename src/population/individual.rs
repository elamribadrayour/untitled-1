use anyhow::Result;
use image::RgbaImage;

use crate::utils::{Assets, Grid};

use crate::population::Gene;

#[derive(Clone)]
pub struct Individual {
    pub id: usize,
    pub genes: Vec<Gene>,
}

impl Individual {
    pub fn new(id: usize, colors: &[usize]) -> Result<Self> {
        if colors.is_empty() {
            return Err(anyhow::anyhow!("colors length is 0"));
        }

        let genes = colors
            .iter()
            .enumerate()
            .map(|(i, color)| Gene::new(i, *color))
            .collect();
        Ok(Self { id, genes })
    }

    pub fn genes(genes: &[Gene], id: usize) -> Self {
        Self {
            id,
            genes: genes.to_vec(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Gene> {
        self.genes.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn save(&self, epoch: usize, assets: &Assets, grid: &Grid) {
        let images = self
            .genes
            .iter()
            .map(|gene| (gene.id, &assets.values[gene.color].image))
            .collect::<Vec<_>>();

        let grid_dim = grid.dim as u32;
        let asset_size = assets.size as u32;
        let (width, height) = (asset_size * grid_dim, asset_size * grid_dim);
        let mut output = RgbaImage::new(width, height);

        for (id, asset) in images.iter() {
            let [x, y] = grid.get(*id);
            let (x_offset, y_offset) = (x as u32 * asset_size, y as u32 * asset_size);
            for (px, py, pixel) in asset.enumerate_pixels() {
                output.put_pixel(px + x_offset, py + y_offset, *pixel);
            }
        }

        output
            .save(format!("results/result-{}.png", epoch))
            .unwrap();
    }
}
