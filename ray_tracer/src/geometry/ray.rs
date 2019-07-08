use crate::geometry::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub o : Vec3,
    pub d : Vec3,
}
impl Ray {
    pub fn new(origin : Vec3, direction : Vec3) -> Self {
        Self {
            o : origin,
            d : direction,
        }
    }
}
impl Ray {
    pub fn point_at_parameter(&self, t : f32) -> Vec3 {
        self.o + t * self.d
    }
}