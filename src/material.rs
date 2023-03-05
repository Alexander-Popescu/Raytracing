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
        albedo: Vec3,
        fuzziness: f32,
    },
    Dielectric {//glass sphere bassically
        ref_index: f32
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
        &Material::Metal { albedo, fuzziness } => {
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction()), &rec.normal());
            *scattered = Ray::ray(rec.p(), reflected + rand_point_in_unit_sphere() * fuzziness);
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal()) > 0.0;
        }
        &Material::Dielectric { ref_index } => {
            let mut ni_over_nt = 0.0;
            let mut outward_normal = Vec3::default();
            let reflected = reflect(&ray_in.direction(), &rec.normal());
            *attenuation = Vec3::new(1.0,1.0,1.0);
            let mut refracted = Vec3::default();

            let mut reflect_prob = 0.0;
            let mut cosine = 0.0;

            if Vec3::dot(&ray_in.direction(), &rec.normal()) > 0.0 {
                outward_normal = -rec.normal();
                ni_over_nt = ref_index;
                cosine = ref_index * Vec3::dot(&ray_in.direction(), &rec.normal()) / ray_in.direction().length()
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_index;
                cosine = -Vec3::dot(&ray_in.direction(), &rec.normal()) / ray_in.direction().length()
            }

            if refract(&ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
                reflect_prob = schlick(cosine, ref_index);
            } else {
                reflect_prob = 1.0;
            }
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                *scattered = Ray::ray(rec.p(), reflected);
            } else {
                *scattered = Ray::ray(rec.p, refracted);
            }
                return true;
        }
    }
}

fn schlick(cosine: f32, ref_index: f32) -> f32 { //vary refraction with angle of ray using schlick approximation
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    return r0 + (1.0-r0) * (1.0 - cosine).powi(5);

}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 { //reflection for metal material
    *v - *n * 2.0 * Vec3::dot(v, n)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {//refraction for dialetrics
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
        return true;
    } else {
        return false;
    }
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