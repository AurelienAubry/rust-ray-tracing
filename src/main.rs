mod vec3;
mod ray;

use crate::vec3::Color;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() -> Result<()> {
    let mut output_file = File::create("image.ppm").context("Failed to create output file")?;
    output_file.write_all(b"P3\n")?;
    output_file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    output_file.write_all(b"255\n")?;

    for row in (0..IMAGE_WIDTH).rev() {
        println!("Remaining row: {}", row);
        for col in 0..IMAGE_HEIGHT {
            let color = Color::new(
                col as f32 / (IMAGE_WIDTH - 1) as f32,
                row as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );

            write_pixel(&mut output_file, &color).context("Failed to write pixel")?;
        }
    }

    Ok(())
}

fn write_pixel(file: &mut File, color: &Color) -> Result<()> {
    let ir = (color.get_x() * 255.0) as u16;
    let ig = (color.get_y() * 255.0) as u16;
    let ib = (color.get_z() * 255.0) as u16;

    Ok(file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?)
}
