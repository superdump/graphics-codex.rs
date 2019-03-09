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

use crate::color::Radiance;

use nalgebra::{Point3, Vector3};

pub struct Surfel {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub radiance: Radiance,
    pub reflectivity: f32,
}

impl Surfel {
    pub fn new(
        position: Point3<f32>,
        normal: Vector3<f32>,
        t: f32,
        radiance: Radiance,
        reflectivity: f32,
    ) -> Surfel {
        Surfel {
            position,
            normal,
            t,
            radiance,
            reflectivity,
        }
    }

    pub fn emitted_radiance(&self, wo: Vector3<f32>) -> Radiance {
        Radiance::new(0f32, 0f32, 0f32)
    }

    pub fn finite_scattering_density(&self, wi: Vector3<f32>, wo: Vector3<f32>) -> f32 {
        if wi.dot(&self.normal) > 0f32 && wo.dot(&self.normal) > 0f32 {
            return self.reflectivity / ::std::f32::consts::PI;
        }
        0f32
    }
}

pub fn surfel(
    position: Point3<f32>,
    normal: Vector3<f32>,
    t: f32,
    radiance: Radiance,
    reflectivity: f32,
) -> Surfel {
    Surfel::new(position, normal, t, radiance, reflectivity)
}
