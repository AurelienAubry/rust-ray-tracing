use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};
use crate::ASPECT_RATIO;
use std::f32::consts::PI;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vertical_fov_deg: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov_deg);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // "z"
        let w = unit_vector(look_from - look_at);
        // "x"
        let u = unit_vector(v_up.cross(&w));
        // "y"
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
