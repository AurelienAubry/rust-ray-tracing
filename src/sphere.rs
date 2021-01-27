use crate::material::Material;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let origin_center = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in acceptable range
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = unit_vector(hit_record.point - self.center);
        hit_record.set_face_normal(&ray, &outward_normal);
        hit_record.material = self.material.clone();
        true
    }
}
