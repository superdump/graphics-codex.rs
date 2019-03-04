use crate::triangle::{IndexedTriangleList, Triangle};

use nalgebra::{Point2, Point3, Vector3};

use std::fmt;

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub triangles: IndexedTriangleList,
    pub translation: Vector3<f32>,
}

impl Mesh {
    pub fn new(
        name: String,
        vertices: Vec<Point3<f32>>,
        uvs: Vec<Point2<f32>>,
        indices: Vec<usize>,
        translation: Vector3<f32>,
    ) -> Mesh {
        Mesh {
            name,
            triangles: IndexedTriangleList::from_vecs(vertices, uvs, indices),
            translation,
        }
    }

    pub fn iter(&self) -> MeshIter {
        MeshIter {
            index: 0,
            mesh: &self,
        }
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type Item = Triangle;
    type IntoIter = MeshIter<'a>;

    fn into_iter(self) -> MeshIter<'a> {
        self.iter()
    }
}

pub struct MeshIter<'a> {
    index: usize,
    mesh: &'a Mesh,
}

impl<'a> Iterator for MeshIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Triangle> {
        if self.index >= self.mesh.triangles.len() {
            return None;
        }
        let triangle = self
            .mesh
            .triangles
            .triangle(self.index)
            .translate(self.mesh.translation);
        self.index = self.index + 1;
        Some(triangle)
    }
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mesh: {}", self.name)?;
        for triangle in &self.triangles {
            write!(f, "{}", triangle)?;
        }
        Ok(())
    }
}

pub fn mesh(
    name: String,
    vertices: Vec<Point3<f32>>,
    uvs: Vec<Point2<f32>>,
    indices: Vec<usize>,
    translation: Vector3<f32>,
) -> Mesh {
    Mesh::new(name, vertices, uvs, indices, translation)
}
