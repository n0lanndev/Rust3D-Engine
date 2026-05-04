use std::ops::{Add, Sub, Mul, Index, IndexMut, Neg, Div, AddAssign, MulAssign};
use std::fmt::{Display, Result, Formatter};

pub type Point3 = Vec3;

#[derive(Default, Copy, Clone)]
pub struct Vec3
{
    pub x: f64, 
    pub y: f64,
    pub z: f64,
}

impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3 {
            x: x,
            y: y, 
            z: z,           
        }
    }

    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Mul<Vec3> for Vec3
{
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3{
        Vec3 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}

impl Div<f64> for Vec3
{
    type Output = Vec3;
    fn div(self, t : f64) -> Vec3{
        self * (1.0 / t)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Mul<f64> for Vec3
{
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3{
        Vec3 {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        }
    }
}

impl Add for Vec3
{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3{
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y, 
            z: self.z + other.z,
        }
    }

}

impl Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3{
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y, 
            z: self.z - other.z,
        }
    }

}

impl Index<usize> for Vec3
{
    type Output = f64;

    fn index(&self, index: usize) -> &f64{
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl Display for Vec3 
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "(x = {}, y = {}, z = {})", self.x, self.y, self.z)
    }
}

// fonction utilitaire

pub fn dot(u: Vec3, v: Vec3) -> f64{
    u.x * v.x + u.y * v.y + u.z * v.z 
}

pub fn unit_vector(u: Vec3) -> Vec3{
    u / u.length()
}

pub fn cross(u: Vec3, v:Vec3) -> Vec3{
    Vec3{
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}