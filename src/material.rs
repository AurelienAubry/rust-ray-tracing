use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Color, Vec3};
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatterable {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool;
}

impl Scatterable for Material {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        match *self {
            Material::Lambertian(ref inner) => {
                inner.scatter(in_ray, hit_record, attenuation, scattered_ray, rng)
            }
            Material::Metal(ref inner) => {
                inner.scatter(in_ray, hit_record, attenuation, scattered_ray, rng)
            }

            Material::Dielectric(ref inner) => {
                inner.scatter(in_ray, hit_record, attenuation, scattered_ray, rng)
            }
        }
    }
}

// ------------
//  LAMBERTIAN
// ------------

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

// -------
//  METAL
// -------

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        let mut f = 1.0;
        if fuzz < 1.0 {
            f = fuzz
        }
        Metal { albedo, fuzz: f }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        let reflected = reflect(unit_vector(in_ray.direction()), hit_record.normal);
        *scattered_ray = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
        );
        *attenuation = self.albedo;
        scattered_ray.direction().dot(&hit_record.normal) > 0.0
    }
}

// ------------
//  DIELECTRIC
// ------------
#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cos: f32, refraction_index_src: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index_src) / (1.0 + refraction_index_src);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        const AIR_REFRACTION_INDEX: f32 = 1.0;

        *attenuation = Color::new(1.0, 1.0, 1.0);
        let (refraction_index_src, refraction_index_dst) = match hit_record.front_face {
            true => (AIR_REFRACTION_INDEX, self.refraction_index),
            false => (self.refraction_index, AIR_REFRACTION_INDEX),
        };

        let refraction_ratio = refraction_index_src / refraction_index_dst;

        let unit_direction = unit_vector(in_ray.direction());
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut direction = Vec3::zero();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>() {
            // Total Reflection
            direction = reflect(unit_direction, hit_record.normal);
        } else {
            // Refract
            direction = refract(
                unit_direction,
                hit_record.normal,
                refraction_index_src,
                refraction_index_dst,
            );
        }

        *scattered_ray = Ray::new(hit_record.point, direction);
        true
    }
}

fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    return vec - 2.0 * vec.dot(&normal) * normal;
}

fn refract(
    i_ray: Vec3,
    normal: Vec3,
    refraction_index_src: f32,
    refraction_index_dst: f32,
) -> Vec3 {
    let etai_over_etat = refraction_index_src / refraction_index_dst;
    let cos_theta = 1.0f32.min(-i_ray.dot(&normal));
    let r_out_perp = etai_over_etat * (i_ray + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    return r_out_perp + r_out_parallel;
}
