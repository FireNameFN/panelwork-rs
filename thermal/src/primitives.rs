use ash::vk::{Extent2D, Extent3D, Offset2D, Rect2D, Viewport};

pub const fn offset(x: i32, y: i32) -> Offset2D {
    Offset2D { x, y }
}

pub const fn extent(width: u32, height: u32) -> Extent2D {
    Extent2D { width, height }
}

pub const fn extent3d(width: u32, height: u32, depth: u32) -> Extent3D {
    Extent3D {
        width,
        height,
        depth,
    }
}

pub const fn rect(x: i32, y: i32, width: u32, height: u32) -> Rect2D {
    Rect2D {
        offset: offset(x, y),
        extent: extent(width, height),
    }
}

pub const fn viewport(x: f32, y: f32, width: f32, height: f32) -> Viewport {
    Viewport {
        x,
        y,
        width,
        height,
        min_depth: 0.,
        max_depth: 0.,
    }
}
