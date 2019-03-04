use nalgebra::{Point2, Point3, Vector3};

pub fn point2(x: f32, y: f32) -> Point2<f32> {
    Point2::new(x, y)
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3::new(x, y, z)
}

pub fn vector3(x: f32, y: f32, z: f32) -> Vector3<f32> {
    Vector3::new(x, y, z)
}
