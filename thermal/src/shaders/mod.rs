use ash::vk::Format;
use ash::vk::VertexInputAttributeDescription;
use ash::vk::VertexInputBindingDescription;
use ash::vk::VertexInputRate;
pub struct SlangShader {
    pub code_bytes: &'static [u8],
    pub attributes: &'static [VertexInputAttributeDescription],
    pub bindings: &'static [VertexInputBindingDescription],
}
impl SlangShader {
    pub fn code(&self) -> &'static [u32] {
        unsafe {
            std::slice::from_raw_parts(
                self.code_bytes.as_ptr() as _,
                self.code_bytes.len() / 4,
            )
        }
    }
}
pub const VERTEX: SlangShader = SlangShader {
    code_bytes: include_bytes!("bin/vertex.spv"),
    attributes: &[
        VertexInputAttributeDescription {
            location: 0u32,
            binding: 0u32,
            format: Format::R32G32_SFLOAT,
            offset: 0u32,
        },
    ],
    bindings: &[
        VertexInputBindingDescription {
            binding: 0u32,
            stride: 8u32,
            input_rate: VertexInputRate::VERTEX,
        },
    ],
};
pub const SOLID: SlangShader = SlangShader {
    code_bytes: include_bytes!("bin/solid.spv"),
    attributes: &[],
    bindings: &[],
};
