use crate::core::{HitRecord, Material};
use crate::geometry::{dot, reflect, refract, Ray, Vec3};
use crate::common::rand;
use crate::core::texture::Texture;
use std::rc::Rc;

pub struct DiffuseLight {
    pub emit : Option<Rc<dyn Texture>>,
}
impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool { false }
    fn emitted(&self, u : f32, v : f32, p : &Vec3) -> Vec3 {
        match &self.emit {
            None => panic!("No emit texture!"),
            Some(e) => return e.value(u,v,p),
        }
    }
}