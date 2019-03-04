use graphics_codex::*;
use std::path::Path;

const MANIFEST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

fn main() {
    triangle_list_example();
    indexed_triangle_list_example();
    obj_example();
}

fn triangle_list_example() {
    let mut mesh = triangle_list();
    let zero2 = point2(0f32, 0f32);

    // Slanted xyz face:
    mesh.append(
        point3(1.0f32, 0.0f32, 0.0f32),
        point3(0.0f32, 1.0f32, 0.0f32),
        point3(0.0f32, 0.0f32, 1.0f32),
        zero2,
        zero2,
        zero2,
    );

    // Bottom zx face:
    mesh.append(
        point3(1.0f32, 0.0f32, 0.0f32),
        point3(0.0f32, 0.0f32, 1.0f32),
        point3(0.0f32, 0.0f32, 0.0f32),
        zero2,
        zero2,
        zero2,
    );

    // Back xy face:
    mesh.append(
        point3(0.0f32, 0.0f32, 0.0f32),
        point3(0.0f32, 1.0f32, 0.0f32),
        point3(1.0f32, 0.0f32, 0.0f32),
        zero2,
        zero2,
        zero2,
    );

    // Left yz face:
    mesh.append(
        point3(0.0f32, 1.0f32, 0.0f32),
        point3(0.0f32, 0.0f32, 0.0f32),
        point3(0.0f32, 0.0f32, 1.0f32),
        zero2,
        zero2,
        zero2,
    );

    for i in 0..4 {
        println!("{:?}", mesh.triangle(i));
    }
}

fn indexed_triangle_list_example() {
    let mut mesh = indexed_triangle_list();
    let zero2 = point2(0f32, 0f32);

    mesh.append_vertex(point3(1.0f32, 0.0f32, 0.0f32), zero2);
    mesh.append_vertex(point3(0.0f32, 1.0f32, 0.0f32), zero2);
    mesh.append_vertex(point3(0.0f32, 0.0f32, 1.0f32), zero2);
    mesh.append_vertex(point3(0.0f32, 0.0f32, 0.0f32), zero2);

    // Slanted xyz face:
    mesh.append(0, 1, 2);

    // Bottom zx face:
    mesh.append(0, 2, 3);

    // Back xy face:
    mesh.append(3, 1, 0);

    // Left yz face:
    mesh.append(1, 3, 2);

    for i in 0..4 {
        println!("{:?}", mesh.triangle(i));
    }
}

fn obj_example() {
    let meshes = load_obj(&Path::new(MANIFEST_DIR).join("assets/models/quad.obj"));

    for mesh in meshes {
        println!("{:?}", mesh);
    }
}
