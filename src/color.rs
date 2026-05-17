use crate::vec3::Vec3;
use crate::interval::Interval;

use std::io::{BufWriter, Write, Stdout};

pub type Color = Vec3;

pub fn write_color(out: &mut BufWriter<Stdout>, pixel_color: &Color) {
    let r: f64 = pixel_color.x;
    let g: f64 = pixel_color.y;
    let b: f64 = pixel_color.z;

    const INTENSITY: Interval = Interval::new(0.000, 0.999);

    let rbyte: u32 = (255.999 * INTENSITY.clamp(r)) as u32;
    let gbyte: u32 = (255.999 * INTENSITY.clamp(g)) as u32;
    let bbyte: u32 = (255.999 * INTENSITY.clamp(b)) as u32;

    let _ = writeln!(out, "{} {} {}", rbyte, gbyte, bbyte);
}