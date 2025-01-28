use image::ImageBuffer;

use crate::utils::Assets;

#[derive(Clone)]
pub struct Gene {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
}

impl Gene {
    pub fn new(assets: &Assets) -> Self {
        Self {
            image: assets.rand(),
        }
    }
}
