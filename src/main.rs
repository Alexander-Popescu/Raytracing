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
mod material;
use material::{scatter, Material};

//rand
extern crate rand;
use rand::prelude::*;


fn main() {

    //various variables

    //resolution must be 2:1
    let width: i32 = 500;
    let height: i32 = 250;

    //maximum value for rgb pixel channels
    let max_color_value: i32 = 255;

    //used to anti-alias by averaging samples
    let samples_per_pixel = 100;
    let mut rand_num = rand::thread_rng();

    //initialize camera
    let camera = Camera::camera();

    //list to hold hittable items (spheres for now)
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    //append spheres to the list
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0),0.5,Material::Lambertian {albedo: Vec3::new(0.8, 0.3, 0.3),},))); // red sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0,Material::Lambertian {albedo: Vec3::new(0.2, 0.2, 0.2),},))); // "ground" sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(1.0, 0.0, -1.0),0.5,Material::Metal {albedo: Vec3::new(0.8, 0.6, 0.2), fuzziness: 0.3},))); // Gold sphere, fuzzy
    list.push(Box::new(Sphere::sphere(Vec3::new(-1.0, 0.0, -1.0),0.5,Material::Dielectric { ref_index: (1.5) },))); //glass sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(-1.0, 0.0, -1.0),-0.45,Material::Dielectric { ref_index: (1.5) },))); //glass sphere with negative radius put inside the other one, to make a hollow glass sphere
    //list.push(Box::new(Sphere::sphere(Vec3::new(-1.0, 0.0, -1.0),0.5,Material::Metal {albedo: Vec3::new(0.8, 0.8, 0.8), fuzziness: 0.0},))); // Silver sphere, perfect reflections

    //take list of hittables and put them in a hittablelist
    let world = HittableList::new(list);

    //boilerplate ppm stuff
    println!("P3\n{} {}\n{}", &width, &height, &max_color_value);

    //loop through all pixels in the image
    for row in (0..height).rev() {
        for column in 0..width {
            //anti-aliasing loop
            let mut col = Vec3::new(0.0,0.0,0.0);
            for _ in 0..samples_per_pixel {
                let u = (column as f32 + rand_num.gen::<f32>()) / (width as f32 - 1.0);
                let v = (row as f32 + rand_num.gen::<f32>()) / (height as f32 - 1.0);

                let r = camera.get_ray(u, v);
                col = col + color(&r, &world, 0);
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
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {

    //check if hit returns something
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        //stop iterating reflections if this pixel is past 50 iterations, otherwise compute color
        if depth < 50 && scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        //render background gradient if nothing was hit
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}