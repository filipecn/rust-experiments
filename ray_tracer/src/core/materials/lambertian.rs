use crate::core::{Material, HitRecord};
use crate::geometry::{Ray, Vec3, random_in_unit_sphere};

pub struct Lambertian {
    albedo : Vec3,
}
impl Lambertian {
    pub fn new(albedo : Vec3) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}