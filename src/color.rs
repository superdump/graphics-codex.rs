use crate::math::vector_pow;

use nalgebra::{Point3, Vector3};

type Color = Point3<f32>;
type Radiance = Vector3<f32>;

pub fn pixel_value(l: Radiance, k: f32, gamma: f32) -> Color {
    // adjust for constant sensitivity
    let l = l / k;
    // maximum radiance at any frequency
    let m = l.x.max(l.y).max(l.z).max(1.0f32);
    // normalize the input
    let l = l / m;
    // restore magnitude but fade towards white when the maximum value approaches 1.0
    let m = ((m - 1.0f32) * 0.2f32).max(0.0f32).min(1.0f32);
    let l = l * (1.0f32 - m) + Radiance::new(m, m, m);
    // gamma encode for a sRGB display
    Color::from(vector_pow(l, 1.0f32 / gamma))
}
