use std::sync::Arc;

use ash::{
    VkResult,
    vk::{PipelineLayout, PipelineLayoutCreateInfo, PushConstantRange},
};

use crate::thvk::{descriptor_set_layout::ThDescriptorSetLayout, device::ThDevice};

pub struct ThPipelineLayout {
    pub handle: PipelineLayout,

    pub device: Arc<ThDevice>,

    pub set_layouts: Vec<Arc<ThDescriptorSetLayout>>,
}

impl ThDevice {
    pub fn create_pipeline_layout(
        self: &Arc<ThDevice>,
        set_layouts: Vec<Arc<ThDescriptorSetLayout>>,
        push_ranges: &[PushConstantRange],
    ) -> VkResult<Arc<ThPipelineLayout>> {
        let set_layouts_ptr = set_layouts
            .iter()
            .map(|set_layout| set_layout.handle)
            .collect::<Vec<_>>();

        let pipeline_layout_info = PipelineLayoutCreateInfo {
            set_layout_count: set_layouts_ptr.len() as u32,
            p_set_layouts: set_layouts_ptr.as_ptr(),
            push_constant_range_count: push_ranges.len() as u32,
            p_push_constant_ranges: push_ranges.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe {
            self.handle
                .create_pipeline_layout(&pipeline_layout_info, None)
        }?;

        Ok(Arc::new(ThPipelineLayout {
            handle: handle,
            device: self.clone(),
            set_layouts: set_layouts,
        }))
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
