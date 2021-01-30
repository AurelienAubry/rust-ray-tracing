mod camera;
mod material;
mod object;
mod ray;
mod sphere;
mod util;
mod vec3;

use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Material, Metal, Scatterable};
use crate::object::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::clamp;
use crate::vec3::{unit_vector, Color, Point3, Vec3};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::Write;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u16 = 400;
pub const IMAGE_HEIGHT: u16 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u16;

pub const SAMPLES_PER_PIXEL: u16 = 100;
pub const BOUNCE_LIMIT: u16 = 50;

fn main() -> Result<()> {
    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(material_ground),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(material_center),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(material_left),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(material_right),
    )));

    // Camera
    let camera = Camera::new();

    // Render
    let mut output_file = File::create("image.ppm").context("Failed to create output file")?;
    output_file.write_all(b"P3\n")?;
    output_file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    output_file.write_all(b"255\n")?;

    let mut rng = rand::thread_rng();

    let progress_bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len} Row, ETA {eta})"),
    );
    for row in (0..IMAGE_HEIGHT).rev().progress_with(progress_bar) {
        //println!("Remaining row: {}", row);
        for col in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (col as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (row as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&mut rng, &ray, &world, BOUNCE_LIMIT);
            }

            write_pixel(&mut output_file, &pixel_color, SAMPLES_PER_PIXEL)
                .context("Failed to write pixel")?;
        }
    }

    Ok(())
}

fn ray_color<H: Hittable>(rng: &mut ThreadRng, ray: &Ray, world: &H, bounce_limit: u16) -> Color {
    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);
    let red: Color = Color::new(1.0, 0.0, 0.0);

    let mut hit_record = HitRecord::empty();

    // If we've exceeded the ray bounce limit, no more light is gathered
    if bounce_limit <= 0 {
        return Color::zero();
    }

    if world.hit(ray, 0.001, f32::MAX, &mut hit_record) {
        let mut scattered = Ray::new(Point3::zero(), Vec3::zero());
        let mut attenuation = Color::zero();

        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered, rng)
        {
            return attenuation * ray_color(rng, &scattered, world, bounce_limit - 1);
        }

        return attenuation;
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * white + t * blue
}

fn write_pixel(file: &mut File, color: &Color, samples_per_pixel: u16) -> Result<()> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f32;

    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    Ok(file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?)
}
