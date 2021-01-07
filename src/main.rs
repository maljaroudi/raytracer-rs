mod vec3;
use vec3::*;
use vec3::color::*;
use vec3::ray::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

        println!("P3\n{} {}\n255", image_width, image_height);
        for j in (0..image_height).rev(){
            for i in 0..image_width{

                let pixel_color: Color = Color::new(i as f32 / (image_width-1) as f32,
                                                    j as f32 / (image_height-1) as f32,
                                                    0.25);

                write_color(pixel_color);


            }
        }


    vec3_tester();




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