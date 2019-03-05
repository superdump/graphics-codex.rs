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

use crate::mesh::Mesh;
use crate::obj::*;
use crate::triangle::Triangle;

use nalgebra::{Point3, Vector3};
use std::path::Path;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub triangles: Vec<Triangle>,
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: Vec::new(),
            planes: Vec::new(),
            triangles: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, path: &Path) {
        let mut obj_meshes = load_obj(path);
        self.meshes.append(&mut obj_meshes);
    }
}

pub fn scene() -> Scene {
    Scene::new()
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

pub fn sphere(center: Point3<f32>, radius: f32) -> Sphere {
    Sphere::new(center, radius)
}

#[derive(Debug)]
pub struct Plane {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Plane {
    pub fn new(p: Point3<f32>, normal: Vector3<f32>) -> Plane {
        Plane { p, normal }
    }
}

pub fn plane(p: Point3<f32>, normal: Vector3<f32>) -> Plane {
    Plane::new(p, normal)
}
