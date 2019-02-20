use nalgebra::Vector3;

pub fn vector_pow(v: Vector3<f32>, exp: f32) -> Vector3<f32> {
    Vector3::new(v.x.powf(exp), v.y.powf(exp), v.z.powf(exp))
}
