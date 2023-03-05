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

    //resolution variables

    let width: i32 = 1200;
    let height: i32 = 800;
    let aspect_ratio = width as f32 / height as f32;

    //maximum value for rgb pixel channels
    let max_color_value: i32 = 255;

    //max depth to avoid infinite light ray bounces
    let max_depth: i32 = 50;

    //used to anti-alias by averaging samples
    let samples_per_pixel: i32 = 500;
    let mut rand_num = rand::thread_rng();

    //initialize camera variables
    let look_from: Vec3 = Vec3::new(13.0,2.0,3.0);//camera position
    let look_at: Vec3 = Vec3::new(0.0,0.0,0.0);//target position
    let v_up: Vec3 = Vec3::new(0.0,1.0,0.0);//up direction for camera
    let focus_distance: f32 = 10.0;//distance to the target (if you want it in focus)
    let aperture: f32 = 0.1;//larger = more depth of field
    let vfov = 20.0;//vertical field of view

    //and the camera
    let camera = Camera::camera(look_from, look_at, v_up, vfov, aspect_ratio, aperture, focus_distance);

    //list to hold hittable items (spheres for now)
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    //large ground sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -1000.0, 0.0),1000.0,Material::Lambertian {albedo: Vec3::new(0.5, 0.5, 0.5),},)));

    //generate random tiny spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand_num.gen::<f32>();
            let center = Vec3::new(a as f32+ 0.9 * rand_num.gen::<f32>(), 0.2, b as f32 + 0.9 * rand_num.gen::<f32>());

            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    //generate lambertian sphere, albedo is squared
                    list.push(Box::new(Sphere::sphere(
                        center,
                        0.2,
                        Material::Lambertian {
                            albedo: Vec3::new(
                                rand_num.gen::<f32>() * rand_num.gen::<f32>(), 
                                rand_num.gen::<f32>() * rand_num.gen::<f32>(), 
                                rand_num.gen::<f32>()) * rand_num.gen::<f32>(),
                        },
                    )));
                } else if choose_material < 0.95 {
                    //generate metal sphere
                    list.push(Box::new(Sphere::sphere(
                        center,
                        0.2,
                        Material::Metal { 
                            albedo: Vec3::new(
                                0.5 * (1.0 + rand_num.gen::<f32>()),
                                0.5 * (1.0 + rand_num.gen::<f32>()),
                                0.5 * (1.0 + rand_num.gen::<f32>()),
                            ), 
                            fuzziness: (0.5 * rand_num.gen::<f32>()) } 
                    )));
                } else {
                    //generate glass sphere
                    list.push(Box::new(Sphere::sphere(
                        center,
                        0.2,
                        Material::Dielectric { ref_index: (1.5) } 
                    )));
                }
            }

        }
    }

    //the big spheres
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 1.0, 0.0),1.0,Material::Dielectric { ref_index: (1.5) },)));
    list.push(Box::new(Sphere::sphere(Vec3::new(-4.0, 1.0, 0.0),1.0,Material::Lambertian {albedo: Vec3::new(0.4, 0.2, 0.1),},)));
    list.push(Box::new(Sphere::sphere(Vec3::new(4.0, 1.0, 0.0),1.0,Material::Metal { albedo: (Vec3::new(0.7, 0.6, 0.5)), fuzziness: (0.0) })));

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
                col = col + color(&r, &world, 0, max_depth);
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

fn color(r: &Ray, world: &HittableList, depth: i32, max_depth: i32) -> Vec3 {

    //check if hit returns something
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        //stop iterating reflections if this pixel is past 50 iterations, otherwise compute color
        if depth < max_depth && scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1, max_depth);
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