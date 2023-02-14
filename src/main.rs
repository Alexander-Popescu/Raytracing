//TODO REWRITE / UNDERSTAND / CLEANUP THE CODE
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
//crates for writing to file
use std::fs::File;
use std::io::Write;


fn main() {
    //create file

    let mut file = File::create("output_images/output.ppm")
        .expect("Could Not create file!");

    //define screen variables

    let width: i32 = 200;
    let height: i32 = 100;
    let max_color_value: i32 = 255;

    //starts rays at lower left corner (-2, -1, -1)

    //writes ppm boilerplate to the file
    write!(file, "P3\n{} {}\n{}\n", &width, &height, &max_color_value)
        .expect("Cannot write to the file");

    let lower_left_corner = Vec3::new(-2.0,-1.0,-1.0); // lower left corner (starting point of raycasts)
    let vertical = Vec3::new(0.0,2.0,0.0); // vertical bound of the camera
    let horizontal = Vec3::new(4.0,0.0,0.0); // horizontal bound of the camera
    let origin = Vec3::new(0.0,0.0,0.0); // camera origin

    let mut list: Vec<Box<dyn Hittable>> = Vec::new(); //mutable array of hittable items
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5))); // add new sphere 1 unit in front of the camera
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0,))); // add another huge sphere far below the camera to appear as a ground
    let world = HittableList::new(list);

    //loop through all pixels in the image
    for row in (0..height).rev() {
        for column in 0..width {
            //choose which pixel we are drawing this iteration
            let u = column as f32 / width as f32;
            let v = row as f32 / height as f32;

            //cast the ray toward that pixel
            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);

            //calculate color for that pixel
            let col = color(&r, &world);

            //scale the color for ppm output
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            //writes each pixel to the file
            write!(file, "{} {} {}\n", ir, ig, ib)
        .expect("Cannot write to the file");
        }
    }
    //print to file or other
    
    println!("Finished Writing To File");

}

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        return Vec3::new(
                rec.normal().x() + 1.0,
                rec.normal().y() + 1.0,
                rec.normal().z() + 1.0,
            ) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}