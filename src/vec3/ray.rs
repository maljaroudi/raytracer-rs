use crate::Point3;
use crate::vec3::Vec3;

#[derive(Debug,Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray{
    pub fn at(self, t: f32) -> Point3{
        self.origin + self.direction*t
    }
}
