use crate::geometry::Vec3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p =
            2.0 * Vec3::new(
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
            ) - Vec3::new(1f32, 1f32, 1f32);
        if p.length2() >= 1f32 {
            break
        }
    }
    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p : Vec3;
    loop {
        p =
            2.0 * Vec3::new(
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                0.0,
            ) - Vec3::new(1f32, 1f32, 0.0);
        if p.length2() >= 1f32 {
            break
        }
    }
    p
}
