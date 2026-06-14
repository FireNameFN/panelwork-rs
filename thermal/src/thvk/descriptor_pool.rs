use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorSet,
        DescriptorSetAllocateInfo, DescriptorSetLayout,
    },
};

use crate::thvk::device::ThDevice;

pub struct ThDescriptorPool {
    pub handle: DescriptorPool,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_descriptor_pool(
        self: &Arc<ThDevice>,
        max_sets: u32,
        sizes: &[DescriptorPoolSize],
    ) -> VkResult<ThDescriptorPool> {
        let descriptor_pool_info = DescriptorPoolCreateInfo {
            max_sets,
            pool_size_count: sizes.len() as u32,
            p_pool_sizes: sizes.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe {
            self.handle
                .create_descriptor_pool(&descriptor_pool_info, None)
        }?;

        Ok(ThDescriptorPool {
            handle,
            device: self.clone(),
        })
    }
}

impl ThDescriptorPool {
    pub fn allocate_descriptor_set(
        &self,
        set_layout: DescriptorSetLayout,
    ) -> VkResult<DescriptorSet> {
        let descriptor_set_info = DescriptorSetAllocateInfo {
            descriptor_pool: self.handle,
            descriptor_set_count: 1,
            p_set_layouts: &set_layout,
            ..Default::default()
        };

        let descriptor_sets = unsafe {
            self.device
                .handle
                .allocate_descriptor_sets(&descriptor_set_info)
        }?;

        Ok(descriptor_sets[0])
    }
}

impl Drop for ThDescriptorPool {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .destroy_descriptor_pool(self.handle, None)
        }
    }
}
