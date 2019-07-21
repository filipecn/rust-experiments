use crate::core::{HitRecord, Material};
use crate::geometry::{dot, normalize, reflect, Ray, Vec3};
use crate::common::random_in_unit_sphere; 

pub struct Metal {
    albedo: Vec3,
    fuzz: f32

}
impl Metal {
    pub fn new(albedo: Vec3, fuzz : f32) -> Self {
        let mut f = 1f32;
        if fuzz < 1.0 {
            f = fuzz;
        }
        Self { albedo, fuzz : f }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let v = normalize(r_in.d);
        let reflected = reflect(&v, &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), r_in.time);
        *attenuation = self.albedo;
        dot(&scattered.d, &rec.normal) > 0f32
    }
}
