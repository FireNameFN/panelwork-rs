use std::sync::Arc;

use ash::{
    VkResult,
    vk::{DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo},
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::device::ThDevice;

#[derive(ThDeviceHandle)]
pub struct ThDescriptorSetLayout {
    handle: DescriptorSetLayout,

    device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_descriptor_set_layout(
        self: &Arc<ThDevice>,
        bindings: &[DescriptorSetLayoutBinding],
    ) -> VkResult<Arc<ThDescriptorSetLayout>> {
        let set_layout_info = DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe {
            self.handle
                .create_descriptor_set_layout(&set_layout_info, None)
        }?;

        Ok(Arc::new(ThDescriptorSetLayout {
            handle: handle,
            device: self.clone(),
        }))
    }
}

impl Drop for ThDescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .destroy_descriptor_set_layout(self.handle, None)
        }
    }
}
