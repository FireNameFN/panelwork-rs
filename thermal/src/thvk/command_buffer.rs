use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        self, AccessFlags, CommandBuffer, CommandBufferBeginInfo, CommandBufferUsageFlags,
        DependencyFlags, Image, ImageAspectFlags, ImageLayout, ImageMemoryBarrier,
        ImageSubresourceRange, PipelineStageFlags,
    },
};

use crate::thvk::device::ThDevice;

pub struct ThCommandBuffer {
    pub handle: CommandBuffer,

    pub device: Arc<ThDevice>,
}

impl ThCommandBuffer {
    pub fn begin(&self, flags: CommandBufferUsageFlags) -> VkResult<()> {
        let begin_info = CommandBufferBeginInfo {
            flags,
            ..Default::default()
        };

        unsafe {
            self.device
                .handle
                .begin_command_buffer(self.handle, &begin_info)
        }
    }

    pub fn end(&self) -> VkResult<()> {
        unsafe { self.device.handle.end_command_buffer(self.handle) }
    }

    pub fn image_barrier(
        &self,
        image: Image,
        src_access: AccessFlags,
        dst_access: AccessFlags,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
        src_stage: PipelineStageFlags,
        dst_stage: PipelineStageFlags,
    ) {
        let image_barrier = ImageMemoryBarrier {
            src_access_mask: src_access,
            dst_access_mask: dst_access,
            old_layout: old_layout,
            new_layout: new_layout,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            image: image,
            subresource_range: ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: vk::REMAINING_MIP_LEVELS,
                base_array_layer: 0,
                layer_count: vk::REMAINING_ARRAY_LAYERS,
            },
            ..Default::default()
        };

        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle,
                src_stage,
                dst_stage,
                DependencyFlags::empty(),
                &[],
                &[],
                &[image_barrier],
            )
        }
    }
}
