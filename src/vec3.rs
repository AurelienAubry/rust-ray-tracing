use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3 {
        Vec3(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        )
    }

    pub fn random_range(rng: &mut ThreadRng, min: f32, max: f32) -> Vec3 {
        Vec3(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let v = Self::random_range(rng, -1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
        unit_vector(Self::random_in_unit_sphere(rng))
    }

    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn is_near_zero(&self) -> bool {
        const EPS: f32 = 1e-8;
        (self.0 < EPS) && (self.1 < EPS) && (self.2 < EPS)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
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

// vecB = v * vecA
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
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
        (1.0 / v) * self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(4.0, 0.0, 3.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(v1.dot(&v2), 140.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.cross(&v2), Vec3(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(4.0, 0.0, 3.0);
        assert_eq!(unit_vector(v), Vec3(0.8, 0.0, 0.6));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, 2.0, 0.0), Vec3(-1.0, -2.0, 0.0));
    }

    #[test]
    fn test_sub() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v - v, Vec3::zero());
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_mul_const() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_const() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v1 * v2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_vec() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        v1 *= v2;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }
}
