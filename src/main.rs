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

pub const ASPECT_RATIO: f32 = 3.0 / 2.0;
pub const IMAGE_WIDTH: u16 = 1200;
pub const IMAGE_HEIGHT: u16 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u16;

pub const SAMPLES_PER_PIXEL: u16 = 500;
pub const BOUNCE_LIMIT: u16 = 50;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    // World
    let world = random_world(&mut rng);

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut output_file = File::create("image.ppm").context("Failed to create output file")?;
    output_file.write_all(b"P3\n")?;
    output_file.write_all(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    output_file.write_all(b"255\n")?;

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
                let ray = camera.get_ray(u, v, &mut rng);
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

fn random_world(rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    let material = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5) as f32;
                    let material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Material::Dielectric(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
