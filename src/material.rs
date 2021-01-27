use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Color, Vec3};
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
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
        }
    }
}

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
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);
        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
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
        *scattered_ray = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;
        scattered_ray.direction().dot(&hit_record.normal) > 0.0
    }
}

fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    return vec - 2.0 * vec.dot(&normal) * normal;
}
