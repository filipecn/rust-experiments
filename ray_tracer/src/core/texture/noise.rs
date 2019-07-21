use crate::common::Perlin;
use crate::core::Texture;
use crate::geometry::{dot, Vec3};
use std::rc::Rc;

pub struct NoiseTexture {
    pub noise: Option<Rc<Perlin>>,
    pub scale: f32,
}
impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        match &self.noise {
            None => panic!("Empty noise field."),
            Some(n) => {
                // return Vec3::new(1.0,1.0,1.0) * n.noise(*p * self.scale),
                // return Vec3::new(1.0,1.0,1.0) * n.turb(*p * self.scale, 7);
                return Vec3::new(1.0, 1.0, 1.0)
                    * 0.5
                    * (1.0 + (self.scale * p.z + 10.0 * n.turb(*p * self.scale, 7)).sin());
            }
        }
    }
}
