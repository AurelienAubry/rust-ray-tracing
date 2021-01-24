mod camera;
mod object;
mod ray;
mod sphere;
mod util;
mod vec3;

use crate::camera::Camera;
use crate::object::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::clamp;
use crate::vec3::{unit_vector, Color, Point3, Vec3};
use anyhow::{Context, Result};
use rand::Rng;
use std::fs::File;
use std::io::Write;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u16 = 400;
pub const IMAGE_HEIGHT: u16 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u16;

pub const SAMPLES_PER_PIXEL: u16 = 100;

fn main() -> Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    let mut output_file = File::create("image.ppm").context("Failed to create output file")?;
    output_file.write_all(b"P3\n")?;
    output_file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    output_file.write_all(b"255\n")?;

    let mut rng = rand::thread_rng();

    for row in (0..IMAGE_HEIGHT).rev() {
        //println!("Remaining row: {}", row);
        for col in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (col as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (row as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            write_pixel(&mut output_file, &pixel_color, SAMPLES_PER_PIXEL)
                .context("Failed to write pixel")?;
        }
    }

    Ok(())
}

fn ray_color<H: Hittable>(ray: &Ray, world: &H) -> Color {
    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);
    let red: Color = Color::new(1.0, 0.0, 0.0);

    let mut hit_record = HitRecord::empty();
    if world.hit(ray, 0.0, f32::MAX, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * white + t * blue
}

fn write_pixel(file: &mut File, color: &Color, samples_per_pixel: u16) -> Result<()> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples_per_pixel as f32;

    r *= scale;
    g *= scale;
    b *= scale;

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    Ok(file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?)
}
