use nalgebra::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }
}

pub fn ray(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
    Ray::new(origin, direction)
}
