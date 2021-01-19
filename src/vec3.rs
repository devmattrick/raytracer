use std::{fmt, ops};

use crate::math::rand_double;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3(f64, f64, f64);

// Convenience aliases
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub const fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn rand(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_double(min, max),
            rand_double(min, max),
            rand_double(min, max),
        )
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::rand(-1.0, 1.0);

            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_in_unit_disk() -> Vec3 {
        loop {
            let p = Self::new(rand_double(-1.0, 1.0), rand_double(-1.0, 1.0), 0.0);

            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_unit() -> Vec3 {
        Self::rand_in_unit_sphere().unit()
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((-self).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = (-(1.0 - r_out_perp.len_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions
        let s = 1e-8;

        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
}

// Operator overloads for Vec3
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.x();
        self.1 -= rhs.y();
        self.2 -= rhs.z();
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.x();
        self.1 *= rhs.y();
        self.2 *= rhs.z();
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

// Operating overloads for f64s
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs * (1.0 / self)
    }
}

// Nice display formatting
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
