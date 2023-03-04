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

    //define screen variables

    let width: i32 = 500;
    let height: i32 = 250;
    let max_color_value: i32 = 255;
    let samples_per_pixel = 100;


    //camera do
    let camera = Camera::camera();

    let mut rand_num = rand::thread_rng();

    let mut list: Vec<Box<dyn Hittable>> = Vec::new(); //mutable array of hittable items

    //add spheres to the list

    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.3, 0.3),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.4, 0.4),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
        },
    )));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
        },
    )));
    let world = HittableList::new(list);//world struct containing all scene objects

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
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
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

fn rand_unit_vector() -> Vec3 {
    Vec3::unit_vector(&rand_point_in_unit_sphere())
}

fn rand_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere: Vec3 = rand_point_in_unit_sphere();
    if Vec3::dot(&in_unit_sphere, &normal) > 0.0 { // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        in_unit_sphere * -1.0
    }
}