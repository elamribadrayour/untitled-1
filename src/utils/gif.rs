use std::fs::File;

use anyhow::Result;
use gif::{Encoder, Frame, Repeat};

pub fn create_gif(epochs: usize) -> Result<()> {
    log::info!("creating gif");

    let img = image::open("results/result-0.png")?.to_rgb8();
    let (width, height) = img.dimensions();

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let mut image = File::create("result.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for epoch in 0..epochs {
        // Load the image from the results directory
        let img = image::open(format!("results/result-{}.png", epoch))?;
        let frame = Frame::from_rgb(width as u16, height as u16, &img.to_rgb8().into_raw()); // Convert to RGB
        encoder.write_frame(&frame).unwrap();
    }
    Ok(())
}
