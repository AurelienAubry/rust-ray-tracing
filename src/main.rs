mod object;
mod ray;
mod sphere;
mod vec3;

use crate::object::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Color, Point3, Vec3};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u16;

const VIEWPORT_WIDTH: f32 = 4.0;
const VIEWPORT_HEIGHT: f32 = VIEWPORT_WIDTH / ASPECT_RATIO;

fn main() -> Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f32 = 1.0;
    let origin = Point3::zero();
    let horizontal_vec = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical_vec = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal_vec / 2.0 - vertical_vec / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut output_file = File::create("image.ppm").context("Failed to create output file")?;
    output_file.write_all(b"P3\n")?;
    output_file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    output_file.write_all(b"255\n")?;

    for row in (0..IMAGE_HEIGHT).rev() {
        //println!("Remaining row: {}", row);
        for col in 0..IMAGE_WIDTH {
            let u = (col as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (row as f32) / (IMAGE_HEIGHT - 1) as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal_vec + v * vertical_vec,
            );
            let color = ray_color(&ray, &world);

            write_pixel(&mut output_file, &color).context("Failed to write pixel")?;
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

fn write_pixel(file: &mut File, color: &Color) -> Result<()> {
    let ir = (color.x() * 255.9) as u8;
    let ig = (color.y() * 255.9) as u8;
    let ib = (color.z() * 255.9) as u8;

    Ok(file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?)
}
