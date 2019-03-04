use crate::color::Radiance;

use nalgebra::{Point3, Vector3};

pub struct Surfel {
    pub x: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub radiance: Radiance,
}

impl Surfel {
    pub fn new(x: Point3<f32>, normal: Vector3<f32>, t: f32, radiance: Radiance) -> Surfel {
        Surfel {
            x,
            normal,
            t,
            radiance,
        }
    }
}

pub fn surfel(x: Point3<f32>, normal: Vector3<f32>, t: f32, radiance: Radiance) -> Surfel {
    Surfel::new(x, normal, t, radiance)
}
