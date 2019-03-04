use crate::point::point3;
use crate::ray::{ray, Ray};

pub struct PinholeCamera {
    pub z_near: f32,
    pub v_fov: f32,
}

impl PinholeCamera {
    pub fn new(z_near: f32, v_fov: f32) -> PinholeCamera {
        PinholeCamera { z_near, v_fov }
    }

    pub fn get_primary_ray(&self, x: f32, y: f32, w: i32, h: i32) -> Ray {
        let side = -2.0f32 * (0.5f32 * self.v_fov).tan();
        let p = point3(
            self.z_near * (x / w as f32 - 0.5f32) * side * w as f32 / h as f32,
            self.z_near * -(y / h as f32 - 0.5f32) * side,
            self.z_near,
        );
        ray(p, p.coords.normalize())
    }
}

pub fn pinhole_camera(z_near: f32, v_fov: f32) -> PinholeCamera {
    PinholeCamera::new(z_near, v_fov)
}
