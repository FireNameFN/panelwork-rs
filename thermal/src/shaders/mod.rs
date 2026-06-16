use std::marker::PhantomData;
use std::sync::Arc;
use ash::VkResult;
use ash::vk::Format;
use ash::vk::DescriptorSetLayoutBinding;
use ash::vk::DescriptorType;
use ash::vk::ShaderStageFlags;
use ash::vk::VertexInputAttributeDescription;
use ash::vk::VertexInputBindingDescription;
use ash::vk::VertexInputRate;
use crate::thvk::device::ThDevice;
use crate::thvk::shader_module::ThShaderModule;
pub struct SlangShader {
    pub code_bytes: &'static [u8],
    pub bindings: &'static [VertexInputBindingDescription],
    pub attributes: &'static [VertexInputAttributeDescription],
    pub set_layouts: &'static [&'static [DescriptorSetLayoutBinding<'static>]],
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
    pub fn create_shader_module(
        &self,
        device: Arc<ThDevice>,
    ) -> VkResult<Arc<ThShaderModule>> {
        device.create_shader_module(self.code_bytes)
    }
}
pub const VERTEX: SlangShader = SlangShader {
    code_bytes: include_bytes!("bin/vertex.spv"),
    bindings: &[
        VertexInputBindingDescription {
            binding: 0u32,
            stride: 8u32,
            input_rate: VertexInputRate::VERTEX,
        },
    ],
    attributes: &[
        VertexInputAttributeDescription {
            location: 0u32,
            binding: 0u32,
            format: Format::R32G32_SFLOAT,
            offset: 0u32,
        },
    ],
    set_layouts: &[],
};
pub const SOLID: SlangShader = SlangShader {
    code_bytes: include_bytes!("bin/solid.spv"),
    bindings: &[
        VertexInputBindingDescription {
            binding: 0u32,
            stride: 0u32,
            input_rate: VertexInputRate::VERTEX,
        },
    ],
    attributes: &[],
    set_layouts: &[],
};
pub const TEXTURE: SlangShader = SlangShader {
    code_bytes: include_bytes!("bin/texture.spv"),
    bindings: &[
        VertexInputBindingDescription {
            binding: 0u32,
            stride: 8u32,
            input_rate: VertexInputRate::VERTEX,
        },
    ],
    attributes: &[
        VertexInputAttributeDescription {
            location: 1u32,
            binding: 0u32,
            format: Format::R32G32_SFLOAT,
            offset: 0u32,
        },
    ],
    set_layouts: &[
        &[
            DescriptorSetLayoutBinding {
                binding: 0u32,
                descriptor_type: DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1u32,
                stage_flags: ShaderStageFlags::FRAGMENT,
                p_immutable_samplers: std::ptr::null(),
                _marker: PhantomData,
            },
        ],
    ],
};
