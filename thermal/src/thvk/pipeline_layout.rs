use std::sync::Arc;

use ash::{
    VkResult,
    vk::{DescriptorSetLayout, PipelineLayout, PipelineLayoutCreateInfo, PushConstantRange},
};

use crate::thvk::device::ThDevice;

pub struct ThPipelineLayout {
    pub handle: PipelineLayout,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_pipeline_layout(
        self: &Arc<ThDevice>,
        set_layouts: &[DescriptorSetLayout],
        push_ranges: &[PushConstantRange],
    ) -> VkResult<ThPipelineLayout> {
        let pipeline_layout_info = PipelineLayoutCreateInfo {
            set_layout_count: set_layouts.len() as u32,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: push_ranges.len() as u32,
            p_push_constant_ranges: push_ranges.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe {
            self.handle
                .create_pipeline_layout(&pipeline_layout_info, None)
        }?;

        Ok(ThPipelineLayout {
            handle: handle,
            device: self.clone(),
        })
    }
}

impl Drop for ThPipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .destroy_pipeline_layout(self.handle, None)
        }
    }
}
