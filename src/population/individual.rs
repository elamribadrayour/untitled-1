use image::RgbaImage;

use crate::utils::Assets;

use crate::population::Gene;

#[derive(Clone)]
pub struct Individual {
    pub id: usize,
    pub genes: Vec<Gene>,
}

impl Individual {
    pub fn new(id: usize, individual_size: usize, assets: &Assets) -> Self {
        Self {
            id,
            genes: (0..individual_size).map(|_| Gene::new(assets)).collect(),
        }
    }

    pub fn genes(genes: &[Gene], id: usize) -> Self {
        Self {
            id,
            genes: genes.to_vec(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn save(&self, batch: usize) {
        println!("saving image bath: {} -- id: {}", batch, self.id);
        let grid_size = (self.len() as f64).sqrt().ceil() as u32;
        let width = 100 * grid_size;
        let height = 100 * grid_size;
        let mut image = RgbaImage::new(width, height);
        let images = self
            .genes
            .iter()
            .map(|gene| gene.image.clone())
            .collect::<Vec<_>>();

        for (i, gene_image) in images.iter().enumerate() {
            let x_offset = (i as u32 % grid_size) * 100;
            let y_offset = (i as u32 / grid_size) * 100;
            for (x, y, pixel) in gene_image.enumerate_pixels() {
                image.put_pixel(x + x_offset, y + y_offset, *pixel);
            }
        }

        image
            .save(format!("results/result-{}-{}.png", self.id, batch))
            .unwrap();
    }
}
