use nalgebra::Vector3;

use std::f32::consts::*;

pub const EPSILON: f32 = 1e-6f32;

pub fn vector_pow(v: Vector3<f32>, exp: f32) -> Vector3<f32> {
    Vector3::new(v.x.powf(exp), v.y.powf(exp), v.z.powf(exp))
}

pub fn radians(degrees: f32) -> f32 {
    PI * degrees / 180.0f32
}

pub fn degrees(radians: f32) -> f32 {
    180.0f32 * radians / PI
}
