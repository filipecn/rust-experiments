use crate::core::texture::Texture;
use crate::geometry::Vec3;
use std::rc::Rc;

pub struct CheckerTexture {
    pub odd: Option<Rc<dyn Texture>>,
    pub even: Option<Rc<dyn Texture>>,
}
impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0f32 {
            match &self.odd {
                None => panic!("Incomplete CheckerTexture!"),
                Some(tex) => return tex.value(u, v, p),
            }
        } else {
            match &self.even {
                None => panic!("Incomplete CheckerTexture!"),
                Some(tex) => return tex.value(u, v, p),
            }
        }
    }
}
