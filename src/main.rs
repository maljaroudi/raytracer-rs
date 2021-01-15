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
use std::fs::File;
use std::io::{Write, Error, BufWriter};
use rayon::prelude::*;
use rayon::iter::FlatMap;
use std::sync::{Arc,Mutex};
use std::borrow::BorrowMut;


fn main() -> Result<(), Error> {


    let aspect_ratio:f32 = 3.0 / 2.0;
    let image_width:i32 = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let world = random_scene();


    let lookfrom: Point3 =  Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.00, 1.00, 0.00);

    let dist_to_focus = 10.0;
    let aperture:f32 = 0.1;

    let cam = Camera::new(aspect_ratio, 20.0, lookfrom,
                          lookat, vup, dist_to_focus, aperture);





    let scene: Vec<Vec<Vec3>> = (0..image_height).into_par_iter().map(|y_rev| {
        let y: f32 = image_height as f32 - y_rev as f32 - 1.0;
        let row: Vec<Vec3> = (0..image_width).into_par_iter().map(|x| {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f32 = (x as f32 + rand::random::<f32>()) / image_width as f32;
                let v: f32 = (y as f32 + rand::random::<f32>()) / image_height as f32;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + ray_color(r, &world, 10);
            }
            color_vector = color_vector/samples_per_pixel as f32;
            color_vector = Vec3::new(color_vector.x().sqrt(), color_vector.y().sqrt(), color_vector.z().sqrt())*255.99;
            color_vector
        }).collect();
        row
            //write_color(bytes,samples_per_pixel,image_width,image_height);
        }).collect();
        write_color(scene,"test.png")?;






        Ok(())




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


fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
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


fn random_scene()  -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian
                                                (Lambertian{
                                                    albedo: Color::new(0.5, 0.5, 0.5) });

    world.add(Sphere{
        center: Vec3::new(0.00, -1000.00, 0.00),
        radius: 1000.00,
        mat_ptr: ground_material
    });


    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_f32(0.0, 1.0);
            let center: Point3 = Point3::new(a as f32+0.9*random_f32(0.0,1.0),
            0.2, b as f32+ 0.9*random_f32(0.0,1.0));

            if (center - Point3::new(4.0,0.2,0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material =
                        Material::Lambertian(Lambertian{albedo});
                    world.add(Sphere{
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    });
                }

                else if choose_mat < 0.95 {
                    let albedo =  Color::random(0.5, 1.0);
                    let fuzz = random_f32(0.00, 0.50);
                    let sphere_material = Material::Metal
                        (Metal{albedo, fuzz });
                    world.add(Sphere{
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    });

                }
                else {
                    let sphere_material =Material::
                                                   Dielectric(Dielectric{index_of_refraction: 1.5}

                    );
                    world.add(Sphere{
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material
                    });
                }
            }

        }

    }
    let material1 = Material::Dielectric(Dielectric{index_of_refraction: 1.5});
    world.add(Sphere{
        center: Vec3::new(0.00, 1.00, 0.00),
        radius: 1.0,
        mat_ptr: material1
    });

    let material2 = Material::Lambertian
                                 (Lambertian{albedo: Color::new(0.4, 0.2, 0.1)})
    ;


    world.add(Sphere{
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material2
    });

    let material3 = Material::Metal(Metal{albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0}
    );
    world.add(Sphere{
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material3
    });

    return world;
}


