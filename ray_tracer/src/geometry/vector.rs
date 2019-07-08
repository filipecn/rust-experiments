use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn length2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }
}
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid Vec3 index!"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid Vec3 index!"),
        }
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, f: f32) -> Self {
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, f: f32) -> Self {
        if f == 0f32 {
            panic!("Cannot divide vector component by zero!");
        }
        Self {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, f: f32) {
        if f == 0f32 {
            panic!("Can't divide by zero!");
        }
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}
pub fn normalize(a: Vec3) -> Vec3 {
    let l = a.length();
    if l == 0f32 {
        panic!("Can't normalize zero vector!");
    }
    a / l
}
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Vec3;
    #[test]
    fn vec3_add() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0
            } + Vec3 {
                x: 2.0,
                y: 3.0,
                z: -1.0
            },
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: -1.0
            }
        );
    }
    #[test]
    fn vec3_idx() {
        let mut v = Vec3::new(1f32, 2f32, 3f32);
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
        let v = Vec3::new(1f32, 1f32, 1f32);
        println!("{:?}", v[3]);
    }
    #[test]
    #[should_panic]
    fn vec3_div_0f32() {
        let v = Vec3::new(1f32, 1f32, 1f32) / 0f32;
    }
}
