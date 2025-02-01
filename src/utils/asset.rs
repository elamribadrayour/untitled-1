use hex_color::HexColor;
use image::{ImageBuffer, Rgba};

#[derive(Clone)]
pub struct Asset {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
}

impl Asset {
    pub fn new(color: &str, image: &ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) -> Self {
        let hex = HexColor::parse(color).unwrap();
        let rgba = Rgba([hex.r, hex.g, hex.b, 255]);
        let mut image = image.clone();
        for pixel in image.pixels_mut() {
            if *pixel.0.last().unwrap() > 0 {
                *pixel = rgba
            }
        }

        Self { image }
    }
}
