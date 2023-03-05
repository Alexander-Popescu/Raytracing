use crate::ray::Ray;
use crate::vec3::Vec3;
extern crate rand;
use rand::prelude::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn camera(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let mut u = Vec3::default();
        let mut v = Vec3::default();
        let mut w = Vec3::default();

        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_of_height = (theta/2.0).tan();
        let half_of_width = aspect_ratio * half_of_height;

        let origin = look_from;
        w = Vec3::unit_vector(&(look_from - look_at));
        u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        v = Vec3::cross(&w, &u);

        let horizontal = u * focus_distance * (2.0 * half_of_width);
        let vertical = v * focus_distance * (2.0 * half_of_height);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_distance;
        let lens_radius = aperture / 2.0;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin, 
            u,
            v,
            w,
            lens_radius
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {

        //defocus blur
        let rd: Vec3 = rand_point_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::ray(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

fn rand_point_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand_num(-1.0, 0.0), rand_num(-1.0,1.0), 0.0);
        if p.squared_length() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

fn rand_num(min: f32, max: f32) -> f32 {
    let rand: f32 = rand::thread_rng().gen_range(min..max);
    return rand
}