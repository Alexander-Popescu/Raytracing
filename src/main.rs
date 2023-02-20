//import vec3
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;
mod hittable;
mod hittable_list;
mod sphere;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
mod camera;
use camera::Camera;
//crates for writing to file
use std::fs::File;
use std::io::Write;

//rand
extern crate rand;
use rand::prelude::*;

fn main() {
    //create file

    let mut file = File::create("output_images/output.ppm")
        .expect("Could Not create file!");

    //define screen variables

    let width: i32 = 400;
    let height: i32 = 200;
    let max_color_value: i32 = 255;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //other variables
    
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(4.0,0.0,0.0);
    let vertical = Vec3::new(0.0,2.0,0.0);
    let lower_left_corner = Vec3::new(-2.0,-1.0,-1.0);


    //camera do
    let camera = Camera::new(origin, horizontal, vertical, lower_left_corner);
    let mut rand_num = rand::thread_rng();

    //writes ppm boilerplate to the file
    write!(file, "P3\n{} {}\n{}\n", &width, &height, &max_color_value)
        .expect("Cannot write to the file");

    let mut list: Vec<Box<dyn Hittable>> = Vec::new(); //mutable array of hittable items
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5))); // add new sphere 1 unit in front of the camera
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0,))); // add another huge sphere far below the camera to appear as a ground
    let world = HittableList::new(list);

    //loop through all pixels in the image
    for row in (0..height).rev() {
        for column in 0..width {
            //anti-aliasing loop
            let mut col = Vec3::new(0.0,0.0,0.0);
            for pixel_sample in 0..samples_per_pixel {
                let u = (column as f32 + rand_num.gen::<f32>()) / (width as f32 - 1.0);
                let v = (row as f32 + rand_num.gen::<f32>()) / (height as f32 - 1.0);
                let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
                col = col + color(&r, &world, max_depth);
            }
            //col = col / samples_per_pixel as f32;
            let scale = 1.0 / samples_per_pixel as f32;

            //sqrt for gamma correction for gamma = 2.0
            let ir = (scale * col.r()).sqrt();
            let ig = (scale * col.g()).sqrt();
            let ib = (scale * col.b()).sqrt();

            //scale so actually visible
            let ir: f32 = 255.99 * ir;
            let ig: f32 = 255.99 * ig;
            let ib: f32 = 255.99 * ib;

            //writes each pixel to the file
            println!("{} {} {}\n", ir, ig, ib);
            write!(file, "{} {} {}\n", ir, ig, ib)
                .expect("Cannot write to the file");
        }
    }
    //print to file or other
    
    println!("Finished Writing To File");

}

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();

    //exit loop after certain number of light bounces
    if depth <= 0 {
        return Vec3::new(0.0,0.0,0.0);
    }

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        let target: Vec3 = rec.p() + rec.normal() + rand_point_in_unit_sphere();
        return color(&Ray::ray(rec.p(), target - rec.p()), world, depth - 1) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn rand_num(min: f32, max: f32) -> f32 {
    let rand: f32 = rand::thread_rng().gen_range(min..max);
    return rand
}

fn rand_point_in_unit_sphere() -> Vec3 {
    loop {
        let point: Vec3 = Vec3::new(rand_num(-1.0, 1.0),rand_num(-1.0, 1.0),rand_num(-1.0, 1.0));
        if point.squared_length() >= 1.0 {
            continue;
        }
        return point;
    }
}