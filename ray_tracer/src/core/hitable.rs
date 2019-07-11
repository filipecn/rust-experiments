use crate::geometry::{dot, Ray, Vec3};
use std::f32;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::new(0f32, 0f32, 0f32),
            normal: Vec3::new(0f32, 0f32, 0f32),
            t: 0f32,
            mat: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitList {
    pub list: Vec<Box<dyn Hitable>>,
}
impl HitList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}
impl Hitable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            mat: rec.mat.clone(),
            ..*rec
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(ray, t_min, closest_so_far, &mut temp_rec) == true {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.mat = temp_rec.mat.clone();
            }
        }
        hit_anything
    }
}

#[derive(Clone)]
pub struct SphereObject {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Option<Rc<dyn Material>>,
}
impl Hitable for SphereObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.o - self.center;
        let a = dot(&ray.d, &ray.d);
        let b = dot(&oc, &ray.d);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0f32 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
        }
        false
    }
}
