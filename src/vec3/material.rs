use crate::vec3::*;
use std::rc::Rc;
use crate::vec3::ray::*;
use crate::vec3::hittable::HitRecord;

pub struct Lambertian {
    pub albedo: Color,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { origin: rec.p, direction: scatter_direction };
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        let mut reflected = Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        *scattered = Ray{ origin: rec.p, direction: reflected};

        *attenuation = self.albedo;

        return scattered.direction.dot(rec.normal) > 0.00
    }
}