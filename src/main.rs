
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
}

fn write_to_ppm(width: i32, height: i32, max_color_value: i32, file: &mut File)
{
    //writes ppm boilerplate to the file
    write!(file, "P3\n{} {}\n{}\n", &width, &height, &max_color_value)
        .expect("Cannot write to the file");
    //loop through all pixels in the image
    for row in 0..height {
        for column in 0..width {
            //calculate color of said pixel
            let r: f32 = column as f32 / width as f32;
            let g: f32 = row as f32 / height as f32;
            let b: f32 = 0.2;

            let ir: f32 = (255.99 * r) as f32;
            let ig: f32 = (255.99 * g) as f32;
            let ib: f32 = (255.99 * b) as f32;

            //writes each pixel to the file
            write!(file, "{} {} {}\n", ir, ig, ib)
        .expect("Cannot write to the file");
        }
    }
    //print to file or other
}


// #[test]
// fn should_fail() {
//     unimplemented!();
// }