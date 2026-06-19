use std::sync::Arc;

use ash::VkResult;
use slang_builder_runtime::CompiledShader;

use crate::thvk::{device::ThDevice, shader_module::ThShaderModule};

impl ThDevice {
    pub fn create_compiled_shader(
        self: &Arc<ThDevice>,
        shader: &CompiledShader,
    ) -> VkResult<ThShaderModule> {
        self.create_shader_module(shader.code)
    }
}
