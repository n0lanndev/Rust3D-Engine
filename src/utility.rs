use std::f64::consts::PI;
use rand::RngExt;

pub const PI_CONST: f64 = PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI_CONST / 180.0
}

#[inline]
pub fn random_double() -> f64{
    rand::rng().random_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64{
    rand::rng().random_range(min..max)
}