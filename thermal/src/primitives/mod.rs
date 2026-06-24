use glam::{Affine2, Vec2, Vec4};

pub mod vk;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub x: f32,

    pub y: f32,

    pub u: f32,

    pub v: f32,

    pub color: Vec4,
}

pub const fn vertex(x: f32, y: f32, u: f32, v: f32, color: Vec4) -> Vertex {
    Vertex { x, y, u, v, color }
}

pub const fn vertex_white(x: f32, y: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        x,
        y,
        u,
        v,
        color: Vec4::ONE,
    }
}

pub const fn viewport_matrix(x: f32, y: f32, width: f32, height: f32) -> Affine2 {
    let w = 2. / width;
    let h = 2. / height;

    Affine2::from_cols(
        Vec2::new(w, 0.),
        Vec2::new(0., h),
        Vec2::new(x * w - 1., y * h - 1.),
    )
}
