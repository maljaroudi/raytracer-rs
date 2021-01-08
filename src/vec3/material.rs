use crate::vec3::*;
use std::rc::Rc;
use crate::vec3::ray::*;
use crate::vec3::hittable::HitRecord;



pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}
