use crate::Color;
use crate::rtweekend::clamp;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32){
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();


    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;





    println!("{} {} {}", 256.00 * clamp(r,0.0,0.999),
             256.00 * clamp(g,0.0,0.999),
             256.00 * clamp(b,0.0,0.999)
    )



}

