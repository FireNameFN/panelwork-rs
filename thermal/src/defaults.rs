use ash::vk::{
    self, ComponentMapping, ComponentSwizzle, ImageAspectFlags, ImageSubresourceLayers,
    ImageSubresourceRange,
};

pub const MAPPING_RGBA: ComponentMapping = ComponentMapping {
    r: ComponentSwizzle::R,
    g: ComponentSwizzle::G,
    b: ComponentSwizzle::B,
    a: ComponentSwizzle::A,
};

pub const SUBRESOURCE_COLOR: ImageSubresourceRange = ImageSubresourceRange {
    aspect_mask: ImageAspectFlags::COLOR,
    base_mip_level: 0,
    level_count: vk::REMAINING_MIP_LEVELS,
    base_array_layer: 0,
    layer_count: vk::REMAINING_ARRAY_LAYERS,
};

pub const SUBRESOURCE_COLOR_LAYER: ImageSubresourceLayers = ImageSubresourceLayers {
    aspect_mask: ImageAspectFlags::COLOR,
    mip_level: 0,
    base_array_layer: 0,
    layer_count: 1,
};
