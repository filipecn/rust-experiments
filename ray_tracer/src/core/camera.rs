use crate::common::{rand, random_in_unit_disk};
use crate::geometry::{cross, normalize, Ray, Vec3};
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new() -> Self {
        let lookfrom = Vec3::new(12.0, 1.2, 4.0);
        let lookat = Vec3::new(2.0, 1.0, 0.0);
        let dist_to_focus = (lookfrom - lookat).length();
        let aperture: f32 = 0.03;
        Camera::build(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            30.0,
            1.0,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        )
    }
    pub fn build(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = v_fov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = normalize(look_from - look_at);
        let u = normalize(cross(&v_up, &w));
        let v = cross(&w, &u);
        Self {
            origin,
            u,
            v,
            lens_radius,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            time0,
            time1,
        }
    }
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + (self.time1 - self.time0) * rand();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
