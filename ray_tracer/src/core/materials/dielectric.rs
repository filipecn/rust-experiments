use crate::core::{HitRecord, Material};
use crate::geometry::{dot, reflect, refract, Ray, Vec3};
use rand::Rng;

pub struct Dielectric {
    ref_idx: f32

}
impl Dielectric {
    pub fn new( ref_idx : f32) -> Self {
        Self { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal : Vec3;
        let reflected = reflect(&r_in.d, &rec.normal);
        let ni_over_nt : f32;
        *attenuation = Vec3::new(1.0,1.0,1.0);
        let mut refracted = Vec3::new(1.0,0.0,0.0);
        let reflect_prob : f32;
        let cosine : f32;
        if dot(&r_in.d, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(&r_in.d, &rec.normal) / r_in.d.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&r_in.d, &rec.normal) / r_in.d.length();
        }
        if refract(&r_in.d, &outward_normal, ni_over_nt, &mut refracted) == true {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }
        let rand_value : f32 = rand::thread_rng().gen();
        if rand_value < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        }
        else {
        }    *scattered = Ray::new(rec.p, refracted);
            
        true
    }
}

fn schlick(cosine : f32, ref_idx : f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
