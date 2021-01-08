//This class is useless (Kinda) All of the implementations are in std rust.
// Check for deletion.

use std::f32;
use rand::Rng;
pub const INFINITY: f32 = f32::INFINITY;

pub const PI: f32 = f32::consts::PI;


pub fn degrees_to_radians(degrees: f32) {
    degrees.to_radians();
}

pub fn random_f32(min: f32, max: f32) -> f32 {
     rand::thread_rng().gen_range(min.. max)
}


pub fn clamp(x: f32, min: f32, max: f32) -> f32{
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

