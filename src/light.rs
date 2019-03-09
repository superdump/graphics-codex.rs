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

use nalgebra::Point3;

pub type Biradiance = Radiance;

pub struct Light {
    pub position: Point3<f32>,
    pub power: Radiance,
}

impl Light {
    pub fn new(position: Point3<f32>, power: Radiance) -> Light {
        Light { position, power }
    }

    pub fn biradiance(&self, x: &Point3<f32>) -> Biradiance {
        self.power / (4f32 * ::std::f32::consts::PI * (self.position - x).magnitude_squared())
    }
}

pub fn light(position: Point3<f32>, power: Radiance) -> Light {
    Light::new(position, power)
}
