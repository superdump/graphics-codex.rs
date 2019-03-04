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
