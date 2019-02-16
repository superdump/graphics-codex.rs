use crate::point::point3;
use crate::triangle::IndexedTriangleList;
use nalgebra::Point3;
use std::path::Path;
use tobj;

pub fn load_obj(path: &Path) -> Vec<Mesh> {
    let mut output = Vec::new();

    let obj = tobj::load_obj(path);
    let (models, _) = obj.unwrap();
    for m in models.iter() {
        let mesh = &m.mesh;
        assert!(mesh.positions.len() % 3 == 0);
        let mut vertices = Vec::with_capacity(mesh.positions.len() % 3);
        for v in 0..mesh.positions.len() / 3 {
            vertices.push(point3(
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2],
            ));
        }
        let mut indices = Vec::with_capacity(mesh.indices.len());
        for i in 0..mesh.indices.len() {
            indices.push(mesh.indices[i] as usize);
        }
        output.push(Mesh::new(m.name.clone(), vertices, indices));
    }

    output
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub triangles: IndexedTriangleList,
}

impl Mesh {
    pub fn new(name: String, vertices: Vec<Point3<f32>>, indices: Vec<usize>) -> Mesh {
        Mesh {
            name,
            triangles: IndexedTriangleList::from_vecs(vertices, indices),
        }
    }
}

pub fn mesh(name: String, vertices: Vec<Point3<f32>>, indices: Vec<usize>) -> Mesh {
    Mesh::new(name, vertices, indices)
}
