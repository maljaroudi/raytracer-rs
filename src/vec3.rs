use std::{ops};
use std::f32;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rtweekend;
pub mod camera;
pub mod material;


#[derive(Debug,Clone, Copy)]
pub struct Vec3 {
    pub e: [f32;3],
}


#[allow(dead_code)]
impl Vec3{
    pub fn new (e0:f32, e1:f32, e2: f32) -> Self {
        Vec3{
            e: [e0,e1,e2],
        }
    }

    pub fn length_squared (&self)->f32{
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length (&self) -> f32{
        self.length_squared().sqrt()
    }


    pub fn x (&self) -> f32{
        self.e[0]
    }

    pub fn y (&self) -> f32{
        self.e[1]
    }

    pub fn z (&self) -> f32{
        self.e[2]
    }
    pub fn dot(&self, rhs: Self) -> f32 {
        self.e[0]*rhs.e[0] + self.e[1]*rhs.e[1] + self.e[2]*rhs.e[2]
    }

    pub fn cross(&self, rhs: Self) -> Self{
        Vec3{
            e: [self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]]
        }


    }


    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random(min: f32, max: f32) -> Self {
        Vec3{
            e: [random_f32(min, max), random_f32(min, max), random_f32(min, max)]
        }
    }

    // if something goes wrong, it's probably here
    pub fn random_in_unit_sphere() -> Self {
        loop{
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.00{
                continue
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop{
            let p = Vec3::new(random_f32(-1.00, 1.00),
                              random_f32(-1.00, 1.00), 0.00);
            if p.length_squared() >= 1.00{
                continue
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn near_zero(self) -> bool {
        let s:f32 = 1e-8;
        return (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s);
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        return v - n * v.dot(n)*2.00;
    }

    pub fn refract(uv: Self, n: Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp: Self =   (uv + n*cos_theta) * etai_over_etat;
        let r_out_parallel: Self =n *  -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        return r_out_perp + r_out_parallel;
    }
    pub fn zeros() -> Self{
        Self{
            e: [0.00,0.00,0.00]
        }
    }
}

impl ops::Add for Vec3{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3{
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]],
        }
    }


}

impl ops::AddAssign for Vec3{

    fn add_assign(&mut self, rhs: Self){
        *self = Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }

    }
}

impl ops::Neg for Vec3{
    type Output = Self;

    fn neg(self) -> Self {
        Vec3{
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, n: f32)->Self{
        Vec3{
            e: [self.e[0]*n, self.e[1]*n, self.e[2]*n]
        }


    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div (self, n: f32) -> Self {
        Vec3{
            e: [self.e[0]/n, self.e[1]/n, self.e[2]/n]

        }
    }
}



impl ops::Sub for Vec3 {

    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3{
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]],
        }
    }


}

//Multiplies every component with its correspondent
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self)->Self{
        Vec3{
            e: [self.e[0]*rhs.e[0], self.e[1]*rhs.e[1], self.e[2]*rhs.e[2]]
        }


    }
}
pub use Vec3 as Color;
pub use Vec3 as Point3;
use crate::vec3::rtweekend::random_f32;

