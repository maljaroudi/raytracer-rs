use crate::rtweekend::*;
use crate::vec3::*;
use crate::vec3::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera{
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,

    }

impl Camera{
    pub fn new() -> Self{
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin: Vec3 = Vec3{ e: [0.00,0.00,0.00] };
        let horizontal: Vec3 = Vec3{ e: [viewport_width,0.00,0.00] };
        let vertical: Vec3 = Vec3{ e: [0.00,viewport_height,0.00] };
        let lower_left_corner: Point3 = origin - horizontal/2.00 - vertical/2.00 -
            Vec3::new(0.00,0.00, focal_length);

        Camera{
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }

    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray{
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner +
                 self.horizontal * u +
                self.vertical * v
                - self.origin,
        }
    }

}

