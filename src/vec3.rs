//vec3 module

//this line adds an implementation for the std::fmt formatting trait by deriving the fmt::Debug implementation so that we can print vectors to std::out
#[derive(Debug, Copy, Clone, PartialEq)]

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

    //getter functions
    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }
    
    pub fn g(self) -> f32 {
        self.e[1]
    }

    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn length(self) -> f32 {
        //takes the square root of the sum of each element in the vector squared
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
}

//overload add operator for vec3
//Because add is a function call (a + b -> a.add(b)), you can overwrite this function when the inputs for the add function are vec3s, in this case,
//we are taking each e value for the vecs, adding them together, and returning a new vec3 with added values of the other two
use std::ops;
impl ops::Add for Vec3 {
    type Output = Self;
    
    fn add(self, rhs: Vec3) -> Self::Output {
        //rhs is right hand side of the operator
        Vec3 { e: [self.e[0] + rhs.e[0],
                    self.e[1] + rhs.e[1],
                    self.e[2] + rhs.e[2]]
            }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Vec3) -> Self::Output {
        //rhs is right hand side of the operator
        Vec3 { e: [self.e[0] - rhs.e[0],
                    self.e[1] - rhs.e[1],
                    self.e[2] - rhs.e[2]]
            }
    }
}

//overload multiply operation for f32 for vec3
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k: f32 = 1.0 / rhs;
        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) + Vec3::new(1.0,1.0,4.0), Vec3::new(3.0,5.0,10.0));
    }

    #[test]
    fn test_vec3_multiply() {
        assert_eq!(Vec3::new(2.0, 3.0, 4.0) * 2.0, Vec3::new(4.0,6.0,8.0));
    }

    #[test]
    fn test_vec3_divide() {
        assert_eq!(Vec3::new(8.0,4.0,2.0) / 2.0, Vec3::new(4.0,2.0,1.0))
    }
}