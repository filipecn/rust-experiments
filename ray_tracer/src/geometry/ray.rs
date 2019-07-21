use crate::geometry::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub o : Vec3,
    pub d : Vec3,
    pub time: f32
}
impl Ray {
    pub fn new(origin : Vec3, direction : Vec3, time : f32) -> Self {
        Self {
            o : origin,
            d : direction,
            time
        }
    }
}
impl Ray {
    pub fn point_at_parameter(&self, t : f32) -> Vec3 {
        self.o + t * self.d
    }
}