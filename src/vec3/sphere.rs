use crate::vec3::{hittable::*, Vec3, Point3, ray::*, material};
use std::rc::Rc;


pub struct Sphere{
    pub center: Point3,
    pub radius: f32,
    pub mat_ptr: Rc<material>,
}

impl Hittable for Sphere{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, mut rec: &mut HitRecord) ->bool {
        let oc: Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.00 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b+sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
            false
        }
        else{
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, outward_normal);
            rec.mat_ptr = Some(self.mat_ptr.clone());
            return true
        }

    }

}



