use nalgebra::{Point2, Point3, Vector3};

use std::fmt;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
    pub uvs: [Point2<f32>; 3],
}

impl Triangle {
    pub fn new(
        p0: Point3<f32>,
        p1: Point3<f32>,
        p2: Point3<f32>,
        uv0: Point2<f32>,
        uv1: Point2<f32>,
        uv2: Point2<f32>,
    ) -> Triangle {
        Triangle {
            vertices: [p0, p1, p2],
            uvs: [uv0, uv1, uv2],
        }
    }

    pub fn translate(&self, translation: Vector3<f32>) -> Triangle {
        triangle(
            self.vertices[0] + translation,
            self.vertices[1] + translation,
            self.vertices[2] + translation,
            self.uvs[0],
            self.uvs[1],
            self.uvs[2],
        )
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "V0 ({}), V1 ({}), V2 ({})",
            self.vertices[0], self.vertices[1], self.vertices[2]
        )
    }
}

pub fn triangle(
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    uv0: Point2<f32>,
    uv1: Point2<f32>,
    uv2: Point2<f32>,
) -> Triangle {
    Triangle::new(p0, p1, p2, uv0, uv1, uv2)
}

pub struct TriangleList {
    vertices: Vec<Point3<f32>>,
    uvs: Vec<Point2<f32>>,
}

impl TriangleList {
    pub fn new() -> TriangleList {
        TriangleList {
            vertices: Vec::new(),
            uvs: Vec::new(),
        }
    }

    pub fn triangle(&self, t: usize) -> Triangle {
        let base_index = 3 * t;
        triangle(
            self.vertices[base_index],
            self.vertices[base_index + 1],
            self.vertices[base_index + 2],
            self.uvs[base_index],
            self.uvs[base_index + 1],
            self.uvs[base_index + 2],
        )
    }

    pub fn len(&self) -> usize {
        self.vertices.len() / 3
    }

    pub fn append(
        &mut self,
        p0: Point3<f32>,
        p1: Point3<f32>,
        p2: Point3<f32>,
        uv0: Point2<f32>,
        uv1: Point2<f32>,
        uv2: Point2<f32>,
    ) {
        self.vertices.push(p0);
        self.vertices.push(p1);
        self.vertices.push(p2);
        self.uvs.push(uv0);
        self.uvs.push(uv1);
        self.uvs.push(uv2);
    }
}

pub fn triangle_list() -> TriangleList {
    TriangleList::new()
}

#[derive(Debug)]
pub struct IndexedTriangleList {
    pub vertices: Vec<Point3<f32>>,
    pub uvs: Vec<Point2<f32>>,
    pub indices: Vec<usize>,
}

impl IndexedTriangleList {
    pub fn new() -> IndexedTriangleList {
        IndexedTriangleList {
            vertices: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_vecs(
        vertices: Vec<Point3<f32>>,
        uvs: Vec<Point2<f32>>,
        indices: Vec<usize>,
    ) -> IndexedTriangleList {
        IndexedTriangleList {
            vertices,
            uvs,
            indices,
        }
    }

    pub fn triangle(&self, t: usize) -> Triangle {
        let base_index = 3 * t;
        triangle(
            self.vertices[self.indices[base_index]],
            self.vertices[self.indices[base_index + 1]],
            self.vertices[self.indices[base_index + 2]],
            self.uvs[self.indices[base_index]],
            self.uvs[self.indices[base_index + 1]],
            self.uvs[self.indices[base_index + 2]],
        )
    }

    pub fn len(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn append_vertex(&mut self, p: Point3<f32>, uv: Point2<f32>) {
        self.vertices.push(p);
        self.uvs.push(uv);
    }

    pub fn append(&mut self, i0: usize, i1: usize, i2: usize) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    pub fn iter(&self) -> IndexedTriangleListIter {
        IndexedTriangleListIter {
            index: 0,
            indexed_triangle_list: &self,
        }
    }
}

impl<'a> IntoIterator for &'a IndexedTriangleList {
    type Item = Triangle;
    type IntoIter = IndexedTriangleListIter<'a>;

    fn into_iter(self) -> IndexedTriangleListIter<'a> {
        self.iter()
    }
}

pub struct IndexedTriangleListIter<'a> {
    index: usize,
    indexed_triangle_list: &'a IndexedTriangleList,
}

impl<'a> Iterator for IndexedTriangleListIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Triangle> {
        if self.index >= self.indexed_triangle_list.len() {
            return None;
        }
        let triangle = self.indexed_triangle_list.triangle(self.index);
        self.index = self.index + 1;
        Some(triangle)
    }
}

pub fn indexed_triangle_list() -> IndexedTriangleList {
    IndexedTriangleList::new()
}
