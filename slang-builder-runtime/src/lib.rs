use ash::vk::{
    DescriptorSetLayoutBinding, VertexInputAttributeDescription, VertexInputBindingDescription,
};

pub struct CompiledShader<'a> {
    pub code: &'a [u8],

    pub vertex_bindings: &'a [VertexInputBindingDescription],

    pub vertex_attributes: &'a [VertexInputAttributeDescription],

    pub set_layouts: &'a [&'a [DescriptorSetLayoutBinding<'a>]],
}
