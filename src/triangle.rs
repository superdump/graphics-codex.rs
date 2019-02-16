use nalgebra::Point3;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
}

impl Triangle {
    pub fn new(p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>) -> Triangle {
        Triangle {
            vertices: [p0, p1, p2],
        }
    }
}

pub fn triangle(p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>) -> Triangle {
    Triangle::new(p0, p1, p2)
}

pub struct TriangleList {
    vertices: Vec<Point3<f32>>,
}

impl TriangleList {
    pub fn new() -> TriangleList {
        TriangleList {
            vertices: Vec::new(),
        }
    }

    pub fn triangle(&self, t: usize) -> Triangle {
        let base_index = 3 * t;
        triangle(
            self.vertices[base_index],
            self.vertices[base_index + 1],
            self.vertices[base_index + 2],
        )
    }

    pub fn size(&self) -> usize {
        self.vertices.len() / 3
    }

    pub fn append(&mut self, p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>) {
        self.vertices.push(p0);
        self.vertices.push(p1);
        self.vertices.push(p2);
    }
}

pub fn triangle_list() -> TriangleList {
    TriangleList::new()
}

#[derive(Debug)]
pub struct IndexedTriangleList {
    vertices: Vec<Point3<f32>>,
    indices: Vec<usize>,
}

impl IndexedTriangleList {
    pub fn new() -> IndexedTriangleList {
        IndexedTriangleList {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_vecs(vertices: Vec<Point3<f32>>, indices: Vec<usize>) -> IndexedTriangleList {
        IndexedTriangleList { vertices, indices }
    }

    pub fn triangle(&self, t: usize) -> Triangle {
        let base_index = 3 * t;
        triangle(
            self.vertices[self.indices[base_index]],
            self.vertices[self.indices[base_index + 1]],
            self.vertices[self.indices[base_index + 2]],
        )
    }

    pub fn size(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn append_vertex(&mut self, p: Point3<f32>) {
        self.vertices.push(p);
    }

    pub fn append(&mut self, i0: usize, i1: usize, i2: usize) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }
}

pub fn indexed_triangle_list() -> IndexedTriangleList {
    IndexedTriangleList::new()
}
