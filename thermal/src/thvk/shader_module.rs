use std::sync::Arc;

use ash::{
    VkResult,
    vk::{ShaderModule, ShaderModuleCreateInfo},
};

use crate::thvk::device::ThDevice;

pub struct ThShaderModule {
    pub handle: ShaderModule,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_shader_module(self: &Arc<ThDevice>, code: &[u32]) -> VkResult<ThShaderModule> {
        let shader_module_info = ShaderModuleCreateInfo {
            code_size: code.len(),
            p_code: code.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_shader_module(&shader_module_info, None) }?;

        Ok(ThShaderModule {
            handle,
            device: self.clone(),
        })
    }
}

impl Drop for ThShaderModule {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_shader_module(self.handle, None) }
    }
}
