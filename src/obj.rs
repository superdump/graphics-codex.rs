use crate::mesh::{mesh, Mesh};
use crate::point::{point2, point3, vector3};
use std::path::Path;
use tobj;

pub fn load_obj(path: &Path) -> Vec<Mesh> {
    let mut output = Vec::new();

    let obj = tobj::load_obj(path);
    let (models, _) = obj.unwrap();
    for model in models.iter() {
        let obj_mesh = &model.mesh;
        assert!(obj_mesh.positions.len() % 3 == 0);
        let mut vertices = Vec::with_capacity(obj_mesh.positions.len() / 3);
        for v in 0..obj_mesh.positions.len() / 3 {
            vertices.push(point3(
                obj_mesh.positions[3 * v],
                obj_mesh.positions[3 * v + 1],
                obj_mesh.positions[3 * v + 2],
            ));
        }
        assert!(obj_mesh.texcoords.len() % 2 == 0);
        let mut uvs = vec![point2(0f32, 0f32); obj_mesh.positions.len() / 3];
        if obj_mesh.texcoords.len() / 2 == obj_mesh.positions.len() / 3 {
            for v in 0..obj_mesh.texcoords.len() / 2 {
                uvs[v] = point2(obj_mesh.texcoords[2 * v], obj_mesh.texcoords[2 * v + 1]);
            }
        }
        let mut indices = Vec::with_capacity(obj_mesh.indices.len());
        for i in 0..obj_mesh.indices.len() {
            indices.push(obj_mesh.indices[i] as usize);
        }
        let gc_mesh = mesh(
            model.name.clone(),
            vertices,
            uvs,
            indices,
            vector3(0f32, 0f32, 0f32),
        );
        println!(
            "Mesh loaded: {} ({} triangles; {} vertices; {} uvs)",
            gc_mesh.name,
            gc_mesh.triangles.len(),
            gc_mesh.triangles.vertices.len(),
            gc_mesh.triangles.uvs.len(),
        );
        output.push(gc_mesh);
    }

    output
}
