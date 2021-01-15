use crate::rtweekend::*;
use crate::vec3::*;
use crate::vec3::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera{
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub vfov: f32,
    pub theta: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub aperture: f32,
    pub focus_dist: f32,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,

    }

impl Camera{
    pub fn new(aspect_ratio: f32, vfov: f32,
               lookfrom: Point3, lookat: Point3
               , vup: Vec3, focus_dist: f32, aperture: f32)->Self{
        let theta:f32 = degrees_to_radians(vfov);
        let h:f32 = (theta/2.00).tan();

        let w = (lookfrom-lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let viewport_height = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let origin: Vec3 = lookfrom;
        let horizontal: Vec3 =   u * viewport_width * focus_dist;
        let vertical: Vec3 =  v * viewport_height * focus_dist;

        let lower_left_corner: Point3 = origin - horizontal/2.00 - vertical/2.00 - w*focus_dist;
        let lens_radius: f32 = aperture / 2.00;
        Camera{
            aspect_ratio,
            viewport_height,
            viewport_width,
            vfov,
            theta,
            lookfrom,
            lookat,
            vup,
            aperture,
            focus_dist,
            u,
            v,
            lens_radius,
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }

    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray{
        let rd: Vec3 =  Vec3::random_unit_vector() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray{
            origin: self.origin + offset,
            direction: self.lower_left_corner +
                 self.horizontal * u +
                self.vertical * v
                - self.origin - offset,
        }
    }

}



