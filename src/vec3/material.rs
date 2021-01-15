use crate::vec3::*;
use crate::vec3::ray::*;
use crate::vec3::hittable::HitRecord;
use crate::vec3::rtweekend::random_f32;
#[derive(Clone)]
pub enum Material{
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}



pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Scatter for Lambertian{
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { origin: rec.p, direction: scatter_direction };
        *attenuation = self.albedo;
        return true;
    }
}
#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}
impl Metal{
    fn new(albedo: Color, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz:fuzz.min(1.0),
        }

    }

}
impl Scatter for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        let mut reflected = Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        *scattered = Ray{ origin: rec.p, direction: reflected+ Vec3::random_in_unit_sphere()* self.fuzz};

        *attenuation = self.albedo;

        return scattered.direction.dot(rec.normal) > 0.00
    }
}
#[derive(Clone)]
pub struct Dielectric {
    pub(crate) index_of_refraction: f32,
}

impl Scatter for Dielectric{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        *attenuation = Color::new(1.0,1.0,1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        }
        else{
            self.index_of_refraction
        };



        let unit_direction: Vec3 = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract ||
            Dielectric::reflectance(cos_theta, refraction_ratio)
                > random_f32(0.00, 1.00) {
            Vec3::reflect(unit_direction, rec.normal)
        }
        else{
            Vec3::refract(unit_direction, rec.normal,refraction_ratio)
        };


        *scattered = Ray{ origin: rec.p, direction };
        return true;
    }
}


impl Scatter for Material{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self{
            Material::Lambertian(ref inner) => {
                inner.scatter(r_in,rec, attenuation, scattered)
            }
            Material::Metal(ref inner) => {
                inner.scatter(r_in,rec, attenuation, scattered)
            }
            Material::Dielectric(ref inner) => {
                inner.scatter(r_in,rec, attenuation, scattered)
            }
        }
    }
}

impl Dielectric{
    fn reflectance ( cosine: f32, ref_idx: f32) -> f32{
        let mut r0 = (1.00-ref_idx) / (1.00+ref_idx);
        r0 *= r0;
        return r0 + (1.00 - r0)* (1.00 - cosine).powi(5);
    }
}