use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Camera {
        Camera {
            origin: origin,
            vertical: vertical,
            horizontal: horizontal,
            lower_left_corner: lower_left_corner
        }
    }
}
//TODO finish the camera idk why it broke