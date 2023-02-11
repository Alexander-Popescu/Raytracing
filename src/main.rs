//vec3 module
mod vec3 {

    //this line adds an implementation for the std::fmt formatting trait by deriving the fmt::Debug implementation so that we can print vectors to std::out
    #[derive(Debug)]

    pub struct Vec3 {
        //this chunk of code creates a vec3 class that was 1 attribute e which is a 3 item array of f32s
        e: [f32;3],
    }

    impl Vec3 {
        pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
            Vec3 {
                //this implementation creates a method Vec3::new() that takes in 3 f32s and will assign them to the 3 items of the array accordingly
                e: [e0, e1, e2]
            }
        }
    }

    //overload add operator for vec3
    //Because add is a function call (a + b -> a.add(b)), you can overwrite this function when the inputs for the add function are vec3s, in this case,
    //we are taking each e value for the vecs, adding them together, and returning a new vec3 with added values of the other two
    use std::ops;
    impl ops::Add for Vec3 {
        type Output = Self;
        
        fn add(self, _rhs: Vec3) -> Vec3 {
            //right now rhs is assumed to be another vector but we can change this later to a nicer sounding name when we go through more of the book, right now it means right hand side, whatever that means
            Vec3 { e: [self.e[0] + _rhs.e[0],
                        self.e[1] + _rhs.e[1],
                        self.e[2] + _rhs.e[2]]
                }
        }
    }
}

//import vec3
use vec3::Vec3;

//crates for writing to file
use std::fs::File;
use std::io::Write;


fn main() {
    //create file

    // let mut file = File::create("output_images/output.ppm")
    //     .expect("Could Not create file!");

    //define screen variables

    let width: i32 = 100;
    let height: i32 = 100;
    let max_color_value: i32 = 255;

    //write_to_ppm(width, height, max_color_value, &mut file);
    //println!("Finished Writing To File");

    let v: Vec3 = Vec3::new(1f32, 2f32, 6f32);
    let v2: Vec3 = Vec3::new(2f32, 6f32, 8f32);

    let v3: Vec3 = v + v2;
    //{:?} lets you input a non standard type for output as long as you have derived the debug implementation
    println!("Added v1, and v2: {:?}", v3);
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