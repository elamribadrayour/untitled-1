use hex_color::HexColor;
use image::imageops::FilterType;
use image::{ImageBuffer, Rgba};

#[derive(Clone)]
pub struct Asset {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
}

impl Asset {
    pub fn new(name: &str, size: usize) -> Self {
        let hex = HexColor::parse(name).unwrap();
        let rgba = Rgba([hex.r, hex.g, hex.b, 255]);
        let mut image = image::open("assets/gene.png")
            .unwrap()
            .resize(size as u32, size as u32, FilterType::Lanczos3)
            .to_rgba8();
        for pixel in image.pixels_mut() {
            if *pixel.0.last().unwrap() > 0 {
                *pixel = rgba
            }
        }

        Self { image }
    }
}
