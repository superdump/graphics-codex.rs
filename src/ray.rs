use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }

    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.origin + t * self.direction
    }
}

pub fn ray(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
    Ray::new(origin, direction)
}
