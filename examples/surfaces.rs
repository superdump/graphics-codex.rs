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
