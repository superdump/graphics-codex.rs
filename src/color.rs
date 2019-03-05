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

use crate::math::vector_pow;

use nalgebra::{Point3, Vector3};

pub type Color = Point3<f32>;
pub type Radiance = Vector3<f32>;

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
