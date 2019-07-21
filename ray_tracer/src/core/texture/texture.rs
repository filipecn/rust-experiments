use crate::geometry::Vec3;

pub trait Texture {
    fn value(&self, u : f32, v : f32, p : & Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color : Vec3,
}
impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: & Vec3) -> Vec3 {
        self.color
    }
}