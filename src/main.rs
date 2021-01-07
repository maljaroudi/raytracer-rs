mod vec3;
use vec3::*;
use vec3::color::*;
use vec3::ray::*;
use vec3::hittable::*;
use crate::vec3::hittable_list::HittableList;
use std::rc::Rc;
use crate::vec3::sphere::*;

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

    // ZA WARDOO
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere{
        center: Point3::new(0.00,0.00,-1.00),
        radius: 0.5
    }));

    world.add(Rc::new(Sphere{
        center: Point3::new(0.00,-100.5, -1.00),
        radius: 100.00
    }));







        println!("P3\n{} {}\n255", image_width, image_height);
        for j in (0..image_height).rev(){
            for i in 0..image_width{
                let u = i as f32 / (image_width-1) as f32;
                let v = j as f32 / (image_height-1) as f32;
                let r = Ray{
                    origin: origin,
                    direction: lower_left_corner + horizontal*u + vertical*v - origin,
                };
                let pixel_color: Color = ray_color(r,&world);
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


fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.00 {
        -1.0
    }
    else{
        (-half_b - discriminant.sqrt()) / a
    }
}


fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord{
        p: Vec3::new(0.00,0.00,0.00),
        normal: Vec3::new(0.00,0.00,0.00),
        t: 0.0,
        front_face: false
    };
    if world.hit(r, 0.00, std::f32::INFINITY, &mut rec){
        return rec.normal+Color::new(1.00,1.00, 1.00) * 0.5;
    }
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.y() +1.0)*0.5;
    Color::new(1.0,1.0,1.0)*(1.0-t) + Color::new(0.5,0.7,1.0)*t



}



