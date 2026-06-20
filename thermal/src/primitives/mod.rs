pub mod vk;

#[derive(Clone)]
pub struct Vertex {
    pub x: f32,

    pub y: f32,

    pub u: f32,

    pub v: f32,
}

pub const fn vertex(x: f32, y: f32, u: f32, v: f32) -> Vertex {
    Vertex { x, y, u, v }
}
