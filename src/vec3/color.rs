use crate::Color;
pub fn write_color(pixel_color: Color){
    println!("{} {} {}", 255.999*pixel_color.x(), 255.999*pixel_color.y(), 255.999*pixel_color.z())

}