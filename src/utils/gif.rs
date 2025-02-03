use std::fs::File;

use anyhow::Result;
use gif::{Encoder, Frame, Repeat};

pub fn create_gif(epochs: usize, data_dir: &str) -> Result<()> {
    let start = 0;
    let end = epochs - 1;
    let nb_epochs = end - start;
    log::info!("creating gif for {} epochs", nb_epochs);

    let img = image::open(format!("{}/type=best/epoch={}/best.png", data_dir, start))?.to_rgb8();
    let (width, height) = img.dimensions();

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let gif_path = format!("{}/result.gif", data_dir);
    let mut image = File::create(gif_path).unwrap();
    let mut encoder = Encoder::new(&mut image, width as u16, height as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Finite(0)).unwrap();

    let step = if nb_epochs > 100 { nb_epochs / 100 } else { 1 };
    for epoch in (start..=end).step_by(step) {
        // Load the image from the results directory
        let img = image::open(format!("{}/type=best/epoch={}/best.png", data_dir, epoch))?;
        let frame = Frame::from_rgb(width as u16, height as u16, &img.to_rgb8().into_raw()); // Convert to RGB
        encoder.write_frame(&frame).unwrap();
    }
    Ok(())
}
