use std::ops;
use std::ops::{Div, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f32, f32, f32);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn get_x(&self) -> f32 {
        self.0
    }

    pub fn get_y(&self) -> f32 {
        self.1
    }

    pub fn get_z(&self) -> f32 {
        self.2
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 + other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

// -vecA
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// vecC = vecA - vecB
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// vecA += vecB
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

// vecC = vecA + vecB
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

// vecB = vecA * v
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Vec3 {
        Vec3(self.0 * v, self.1 * v, self.2 * v)
    }
}

// vecA *= v
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, v: f32) {
        self.0 *= v;
        self.1 *= v;
        self.2 *= v;
    }
}

// vecA *= vecB
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

// vecC = vecA * vecB
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

// vecB = vecA / v
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Self::Output {
        Vec3(self.0 / v, self.1 / v, self.2 / v)
    }
}

// vecA /= v
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, v: f32) {
        self.0 /= v;
        self.1 /= v;
        self.2 /= v;
    }
}
