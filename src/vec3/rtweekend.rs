//This class is useless (Kinda) All of the implementations are in std rust.
// Check for deletion.

use std::f32;
use rand::{Rng, SeedableRng, thread_rng};
use rand::rngs::SmallRng;


pub const INFINITY: f32 = f32::INFINITY;

pub const PI: f32 = f32::consts::PI;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees.to_radians()
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    let mut thread_rng = thread_rng();

    let mut small_rng = SmallRng::from_rng(&mut thread_rng).unwrap();
    let random:f32 = small_rng.gen_range(min..max);
     return random;
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

