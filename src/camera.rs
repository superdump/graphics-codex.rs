/* MIT License
 *
 * Copyright (c) 2019 Robert Swain <robert.swain@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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
