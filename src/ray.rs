use crate::vec3::Vec3;

struct Ray {
    A: Vec3,
    B: Vec3
}

impl Ray {
    fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray {
            A: a,
            B: b
        }
    }

    fn origin () -> Vec3 {
        A
    }

    fn direction() -> Vec3 {
        B
    }

    fn point_at_parameter(t: f32) -> Vec3 {
        A + B * t
    }
}