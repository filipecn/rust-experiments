use crate::geometry::{dot, normalize, Vec3};
use rand::Rng;

pub fn rand() -> f32 {
    rand::thread_rng().gen()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand(), rand(), rand()) - Vec3::new(1f32, 1f32, 1f32);
        if p.length2() >= 1f32 {
            break;
        }
    }
    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand(), rand(), 0.0) - Vec3::new(1f32, 1f32, 0.0);
        if p.length2() >= 1f32 {
            break;
        }
    }
    p
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0f32;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}

pub struct Perlin {
    ranvec: Vec<Vec3>,
    ranfloat: Vec<f32>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat: Vec<f32> = vec![];
        let mut ranvec: Vec<Vec3> = vec![];
        let mut perm_x: Vec<usize> = vec![];
        let mut perm_y: Vec<usize> = vec![];
        let mut perm_z: Vec<usize> = vec![];
        perlin_generate_f(&mut ranfloat);
        perlin_generate_v(&mut ranvec);
        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);
        Self {
            ranvec,
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }
    pub fn turb(&self, p: Vec3, depth: i32) -> f32 {
        let mut accum = 0f32;
        let mut temp_p = p;
        let mut weight = 1f32;
        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2f32;
        }
        accum.abs()
    }
}

fn perlin_generate_f(p: &mut Vec<f32>) {
    for _i in 0..256 {
        p.push(rand());
    }
}

fn perlin_generate_v(p: &mut Vec<Vec3>) {
    for _i in 0..256 {
        p.push(normalize(Vec3::new(
            -1.0 + 2.0 * rand(),
            -1.0 + 2.0 * rand(),
            -1.0 + 2.0 * rand(),
        )));
    }
}

fn permute(p: &mut Vec<usize>) {
    for i in (1..(p.len() - 1)).rev() {
        let target = (rand() * (i + 1) as f32) as usize;
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_generate_perm(p: &mut Vec<usize>) {
    for i in 0..256 {
        p.push(i);
    }
    permute(p);
}
