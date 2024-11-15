use super::interval::Interval;
use super::utils::{rnd_float, rnd_float_range};
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        };
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        };
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn rand_unit_vector() -> Vec3 {
    loop {
        let p: Vec3 = random_range_vec(-1.0, 1.0);
        let lensq: f64 = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_vec() -> Vec3 {
    return Vec3 {
        x: rnd_float(),
        y: rnd_float(),
        z: rnd_float(),
    };
}

pub fn random_range_vec(min: f64, max: f64) -> Vec3 {
    return Vec3 {
        x: rnd_float_range(min, max),
        y: rnd_float_range(min, max),
        z: rnd_float_range(min, max),
    };
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = rand_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn write_color(&self) -> String {
        let intensity: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let r = (256.0 * intensity.clamp(self.x)) as u8;
        let g = (256.0 * intensity.clamp(self.y)) as u8;
        let b = (256.0 * intensity.clamp(self.z)) as u8;

        format!("{} {} {}\n", r, g, b)
    }
}
