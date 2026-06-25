use glam::Vec4;

use crate::{
    mesh::Mesh,
    primitives::{Vertex, vertex},
};

pub struct Rect {
    pub vertex1: Vertex,

    pub vertex2: Vertex,

    pub vertex3: Vertex,

    pub vertex4: Vertex,
}

impl Rect {
    pub const fn new(x: f32, y: f32, x2: f32, y2: f32, color: Vec4) -> Rect {
        Rect {
            vertex1: vertex(x, y, 0., 0., color),
            vertex2: vertex(x2, y, 1., 0., color),
            vertex3: vertex(x, y2, 0., 1., color),
            vertex4: vertex(x2, y2, 1., 1., color),
        }
    }

    pub const fn new_white(x: f32, y: f32, x2: f32, y2: f32) -> Rect {
        Self::new(x, y, x2, y2, Vec4::ONE)
    }
}

impl Mesh<Vertex, 6> for Rect {
    fn vertices(&self) -> [Vertex; 6] {
        [
            self.vertex1,
            self.vertex2,
            self.vertex3,
            self.vertex2,
            self.vertex3,
            self.vertex4,
        ]
    }
}
