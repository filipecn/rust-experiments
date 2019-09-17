use std::f32;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}
impl Index<usize> for Point3 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid Point3 index!"),
        }
    }
}
impl IndexMut<usize> for Point3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid Point3 index!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ponos::geometry::Point3;
    #[test]
    fn vec3_idx() {
        let mut v = Point3::new(1f32, 2f32, 3f32);
        assert_eq!(v[0], 1f32);
        assert_eq!(v[1], 2f32);
        assert_eq!(v[2], 3f32);
        v[0] *= -1f32;
        v[1] *= -1f32;
        v[2] *= -1f32;
        assert_eq!(v[0], -1f32);
        assert_eq!(v[1], -2f32);
        assert_eq!(v[2], -3f32);
    }
    #[test]
    #[should_panic]
    fn vec3_inv_idx() {
        let v = Point3::new(1f32, 1f32, 1f32);
        println!("{:?}", v[3]);
    }
}
