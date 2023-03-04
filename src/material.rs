use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
extern crate rand;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3
    },
    Dielectric {

    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian{ albedo: Vec3::default() }
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p() + rec.normal() + rand_point_in_unit_sphere();
            *scattered = Ray::ray(rec.p(), target - rec.p());
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo } => {
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction()), &rec.normal());
            *scattered = Ray::ray(rec.p(), reflected);
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;
        }
        &Material::Dielectric {  } => {
            false
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * Vec3::dot(v, n)
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

fn rand_num(min: f32, max: f32) -> f32 {
    let rand: f32 = rand::thread_rng().gen_range(min..max);
    return rand
}