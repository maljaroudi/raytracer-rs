use std::rc::Rc;
use crate::hittable::*;
use crate::vec3::ray::Ray;
use crate::vec3::Vec3;

pub struct HittableList{
   pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList{
    pub fn add(&mut self,object: Rc<dyn Hittable>) {
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

impl Hittable for HittableList{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec: HitRecord = HitRecord{
            p: Vec3::new(0.00,0.00,0.00),
            normal: Vec3::new(0.00,0.00,0.00),
            t: 0.0,
            front_face: false
        };

        for i in self.objects.iter(){
            if i.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;

    }
}
