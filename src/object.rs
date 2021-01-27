use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            point: Point3::zero(),
            normal: Vec3::zero(),
            material: Material::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // If the ray is inside the object, the ray and the outward normal are in the same direction
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone()
        } else {
            self.normal = -outward_normal.clone()
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut tmp_hit_record = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut tmp_hit_record) {
                hit_anything = true;
                closest_so_far = tmp_hit_record.t;
                *hit_record = tmp_hit_record;
            }
        }

        hit_anything
    }
}
