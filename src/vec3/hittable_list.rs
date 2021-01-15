use std::rc::Rc;
use crate::hittable::*;
use crate::vec3::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
use crate::vec3::sphere::Sphere;

pub struct HittableList{
   pub objects: Vec<Sphere>
}

impl HittableList{
    pub fn add(&mut self,object: Sphere) {
        self.objects.push(object)
    }
    pub fn clear(&mut self){
        self.objects = vec![]
    }
    pub fn new() -> HittableList{
        HittableList{
            objects: vec![]
        }
    }
}
impl HittableList{
    pub(crate) fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let mut hit_anything = false;
        let mut closest_so_far = t_max;


        for i in self.objects.iter(){
            let mut temp_rec: HitRecord = HitRecord{
                p: Vec3::new(0.00,0.00,0.00),
                normal: Vec3::new(0.00,0.00,0.00),
                mat_ptr: None,
                t: 0.0,
                front_face: false
            };
            if i.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;

    }
}
