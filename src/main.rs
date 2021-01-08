mod vec3;
use vec3::*;
use vec3::color::*;
use vec3::ray::*;
use vec3::hittable::*;
use crate::vec3::hittable_list::HittableList;
use std::rc::Rc;
use vec3::sphere::*;
use vec3::camera::*;
use vec3::rtweekend::*;
use vec3::material::*;
fn main() {


    let aspect_ratio:f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // ZA WARDOO
    let mut world = HittableList::new();


    let material_ground = Rc::new(Lambertian{
        albedo: Color{ e: [0.8,0.8,0.0] }
    });

    let material_center = Rc::new(Lambertian{
        albedo: Color{ e: [0.7, 0.3, 0.3] }
    });

    let material_left = Rc::new(Metal{
        albedo: Color{ e: [0.8, 0.8, 0.8] }
    });
    let material_right = Rc::new(Metal{
        albedo: Color{ e: [0.8, 0.6, 0.2] }
    });

    world.add(Rc::new(Sphere{
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground}
            )
    );



    world.add(Rc::new(Sphere{
        center: Point3::new(0.0,    0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center}
    )
    );

    world.add(Rc::new(Sphere{
        center: Point3::new(-1.0,    0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_left}
    )
    );

    world.add(Rc::new(Sphere{
        center: Point3::new( 1.0,    0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right}
    )
    );


    let cam = Camera::new();





        println!("P3\n{} {}\n255", image_width, image_height);
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                let mut pixel_color: Color = Color::new(0.00, 0.00, 0.00);
                for s in 0..samples_per_pixel {
                    let u = (i as f32 + random_f32(0.00, 1.00)) / (image_width - 1) as f32;
                    let v = (j as f32 + random_f32(0.00, 1.00)) / (image_height - 1) as f32;
                    let r: Ray = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &world, max_depth);
                }
                write_color(pixel_color, samples_per_pixel);
            }
        }

    //vec3_tester();




}



fn vec3_tester(){

    let v1 = Vec3::new(1.00, 2.00, 3.00);
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


fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord{
        p: Vec3::new(0.00,0.00,0.00),
        normal: Vec3::new(0.00,0.00,0.00),
        mat_ptr: None,
        t: 0.0,
        front_face: false
    };

    if depth <=0 {
        return Color::new(0.00,0.00,0.00);
    }

    if world.hit(r, 0.001, std::f32::INFINITY, &mut rec){
        let mut scattered = Ray{ origin: Vec3::new(0.0,0.0,0.0)
            , direction: Vec3::new(0.0,0.0,0.0) };
        let mut attenuation: Color = Color::new(0.0,0.0,0.0);


        if rec.mat_ptr.as_ref().unwrap().scatter(&r,&rec, &mut attenuation, & mut scattered ){
            return attenuation * ray_color(scattered, world, depth-1);

        }
        else{
            return Color::new(0.00,0.00,0.00);
        }

    }
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.y() +1.0)*0.5;
    Color::new(1.0,1.0,1.0)*(1.0-t) + Color::new(0.5,0.7,1.0)*t



}



