use crate::geometry::{cross, normalize, Ray, Vec3};
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f32, aspect: f32) -> Self {
        let theta = v_fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = normalize(look_from - look_at);
        let u = normalize(cross(&v_up, &w));
        let v = cross(&w, &u);
        Self {
            origin: origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2f32 * half_width * u,
            vertical: 2f32 * half_height * v,
        }
    }
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
