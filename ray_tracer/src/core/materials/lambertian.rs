use crate::core::{HitRecord, Material};
use crate::geometry::{Ray, Vec3};
use crate::common::random_in_unit_sphere;
use crate::core::Texture;
use std::rc::Rc;

pub struct Lambertian {
    albedo: Option<Rc<dyn Texture>>,
}
impl Lambertian {
    pub fn new(albedo: Option<Rc<dyn Texture>>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        match &self.albedo {
            None => panic!("Material without texture!"),
            Some(v) => *attenuation = v.value(0.0,0.0,&rec.p),
        }
        true
    }
}
