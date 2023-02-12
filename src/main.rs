//import vec3
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

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

    write_to_ppm(width, height, max_color_value, &mut file);
    println!("Finished Writing To File");

}

fn write_to_ppm(width: i32, height: i32, max_color_value: i32, file: &mut File)
{
    //starts rays at lower left corner (-2, -1, -1)

    //writes ppm boilerplate to the file
    write!(file, "P3\n{} {}\n{}\n", &width, &height, &max_color_value)
        .expect("Cannot write to the file");

    let lower_left_corner = Vec3::new(-2.0,-1.0,-1.0);
    let vertical = Vec3::new(0.0,2.0,0.0);
    let horizontal = Vec3::new(4.0,0.0,0.0);
    let origin = Vec3::new(0.0,0.0,0.0);

    //loop through all pixels in the image
    for row in (0..height).rev() {
        for column in 0..width {
            //calculate color of said pixel
            let u: f32 = column as f32 / width as f32;
            let v: f32 = row as f32 / height as f32;
            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = color(&r);

            let ir: f32 = (255.99 * color.r()) as f32;
            let ig: f32 = (255.99 * color.g()) as f32;
            let ib: f32 = (255.99 * color.b()) as f32;

            //writes each pixel to the file
            write!(file, "{} {} {}\n", ir, ig, ib)
        .expect("Cannot write to the file");
        }
    }
    //print to file or other
}

fn color(r: &Ray) -> Vec3 { //takes ray, turns direction vector into unit vector, does lerp function -> (1 - t) * startValue + t * endValue
    //check if hit sphere, set color to the normal of the point
    let t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    //background color of no other returns

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (Vec3::new(1.0,1.0,1.0) * (1.0 - t)) + (Vec3::new(0.5,0.7,1.0) * t)
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 { //checks if an input ray r will intersect with a sphere centered at center with radius radius and returns point
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}