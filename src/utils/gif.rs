use std::fs::File;

use anyhow::Result;
use gif::{Encoder, Frame, Repeat};

pub fn create_gif(epochs: usize) -> Result<()> {
    log::info!("creating gif");
    let start = 1;

    let img = image::open(format!("results/result-{}.png", start))?.to_rgb8();
    let (width, height) = img.dimensions();

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = File::create("result.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Finite(0)).unwrap();

    for epoch in start..=epochs {
        // Load the image from the results directory
        let img = image::open(format!("results/result-{}.png", epoch))?;
        let frame = Frame::from_rgb(width as u16, height as u16, &img.to_rgb8().into_raw()); // Convert to RGB
        encoder.write_frame(&frame).unwrap();
    }
    Ok(())
}
