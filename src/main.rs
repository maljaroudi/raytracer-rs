mod vec3;
use vec3::*;
use vec3::color::*;
use vec3::ray::*;

fn main() {


    let aspect_ratio:f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let focal_length: f32 = 1.0;

    let origin = Point3{e: [0.00,0.00,0.00]};
    let horizontal = Vec3 {e: [viewport_width,0.00,0.00]};
    let vertical = Vec3 {e: [0.00,viewport_height,0.00]};

    let lower_left_corner = origin - horizontal/2.00 - vertical/2.00 -
        Vec3{e: [0.00, 0.00, focal_length]};








        println!("P3\n{} {}\n255", image_width, image_height);
        for j in (0..image_height).rev(){
            for i in 0..image_width{
                let u = i as f32 / (image_width-1) as f32;
                let v = j as f32 / (image_height-1) as f32;
                let r = Ray{
                    origin: origin,
                    direction: lower_left_corner + horizontal*u + vertical*v - origin,
                };
                let pixel_color: Color = ray_color(r);
                write_color(pixel_color);

            }
        }


    //vec3_tester();




}



fn vec3_tester(){
    let  v1 = Vec3::new(1.00, 2.00, 3.00);
    let v2 = Vec3::new(1.00, 1.00, 1.00);
    let v3: Vec3 = v1+v2;
    println!("{:?}", v3.e);
    let v5 = Vec3::new(1.00, 2.00, 3.00);
    let v4: Vec3 = v5*2.00;
    println!("Scaler = {:?}", v4.e);

    let v6 = v4.unit_vector();
    println!("Unit Vector of the Scaler: {:?}",v6.e);

    println!("the x component of the scalar is {}, y is {}, z is {}", v6.x(), v6.y(), v6.z());


    println!("Dot Product: {}",v1.dot(v2));
    println!("Cross product between scalar and v1 {:?}", v1.cross(v4));


    println!("Normal multiplication {:?}", v1*v1);


    println!("Testing Rays ...");
    let r1 = Ray{
    origin: Vec3{e: [1.00,2.00,3.00]},
    direction: Vec3{e: [1.00,2.00,3.00]}
    };

    println!("{:?}", r1.at(1.00));

}



fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Color{e: [1.0, 1.0, 1.0]} * (1.0-t) + Color{e: [0.5, 0.7, 1.0]}*t ;

}