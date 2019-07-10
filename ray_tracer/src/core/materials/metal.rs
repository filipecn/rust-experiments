use crate::core::{Material, HitRecord};
use crate::geometry::{Ray, Vec3, normalize, dot, reflect};

pub struct Metal {
    albedo : Vec3,
}
impl Metal {
    pub fn new(albedo : Vec3) -> Self {
        Self {
            albedo
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Vec3, scattered : &mut Ray) -> bool {
        let v = normalize(r_in.d);
        let reflected = reflect(&v, &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(&scattered.d, &rec.normal) > 0f32
    }
}