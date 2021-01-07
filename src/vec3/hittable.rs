use crate::vec3::*;
use crate::vec3::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
         match r.direction.dot(outward_normal) < 0.00{
             true => {  self.front_face = true;
                        self.normal = outward_normal;
                        },
             false => {
                 self.front_face = false;
                 self.normal = -outward_normal;
             }
         }

    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;

}