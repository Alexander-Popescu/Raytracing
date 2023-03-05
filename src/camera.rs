use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn camera(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f32, aspect: f32) -> Camera {
        let mut u = Vec3::default();
        let mut v = Vec3::default();
        let mut w = Vec3::default();

        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_of_height = (theta/2.0).tan();
        let half_of_width = aspect * half_of_height;

        let origin = look_from;
        w = Vec3::unit_vector(&(look_from - look_at));
        u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: origin - u * half_of_width - v * half_of_height - w,
            horizontal: u * 2.0 * half_of_width,
            vertical: v * 2.0 * half_of_height,
            origin: origin,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::ray(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
//TODO finish the camera idk why it broke