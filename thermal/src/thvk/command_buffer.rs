use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        self, AccessFlags, Buffer, ClearValue, CommandBuffer, CommandBufferBeginInfo,
        CommandBufferUsageFlags, DependencyFlags, DescriptorSet, Framebuffer, Image,
        ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, Pipeline,
        PipelineBindPoint, PipelineLayout, PipelineStageFlags, Rect2D, RenderPass,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo, Viewport,
    },
};

use crate::thvk::{
    command_pool::ThCommandPool,
    device::ThDevice,
    handle::{ThDeviceHandle, ThHandle},
};

pub struct ThCommandBuffer {
    pub handle: CommandBuffer,

    pub command_pool: Arc<ThCommandPool>,
}

impl ThHandle<CommandBuffer> for ThCommandBuffer {
    fn handle(&self) -> CommandBuffer {
        self.handle
    }
}

impl ThDeviceHandle<CommandBuffer> for ThCommandBuffer {
    fn device(&self) -> &Arc<ThDevice> {
        self.command_pool.device()
    }
}

impl ThCommandBuffer {
    pub fn begin(&self, flags: CommandBufferUsageFlags) -> VkResult<()> {
        let begin_info = CommandBufferBeginInfo {
            flags,
            ..Default::default()
        };

        unsafe {
            self.device()
                .handle
                .begin_command_buffer(self.handle, &begin_info)
        }
    }

    pub fn end(&self) -> VkResult<()> {
        unsafe { self.device().handle.end_command_buffer(self.handle) }
    }

    pub fn cmd_begin_render_pass(
        &self,
        render_pass: RenderPass,
        framebuffer: Framebuffer,
        render_area: Rect2D,
        clear_values: &[ClearValue],
        contents: SubpassContents,
    ) {
        let render_pass_info = RenderPassBeginInfo {
            render_pass,
            framebuffer,
            render_area,
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
            ..Default::default()
        };

        let subpass_info = SubpassBeginInfo {
            contents,
            ..Default::default()
        };

        unsafe {
            self.device().handle.cmd_begin_render_pass2(
                self.handle,
                &render_pass_info,
                &subpass_info,
            )
        }
    }

    pub fn cmd_end_render_pass(&self) {
        unsafe {
            self.device()
                .handle
                .cmd_end_render_pass2(self.handle, &SubpassEndInfo::default())
        }
    }

    pub fn cmd_set_viewport(&self, viewport: Viewport) {
        unsafe {
            self.device()
                .handle
                .cmd_set_viewport(self.handle, 0, &[viewport])
        }
    }

    pub fn cmd_set_scissor(&self, scissor: Rect2D) {
        unsafe {
            self.device()
                .handle
                .cmd_set_scissor(self.handle, 0, &[scissor])
        }
    }

    pub fn cmd_bind_pipeline(&self, pipeline: Pipeline) {
        unsafe {
            self.device().handle.cmd_bind_pipeline(
                self.handle,
                PipelineBindPoint::GRAPHICS,
                pipeline,
            )
        }
    }

    pub fn cmd_bind_vertex_buffers(&self, first_binding: u32, buffers: &[Buffer], offsets: &[u64]) {
        unsafe {
            self.device().handle.cmd_bind_vertex_buffers(
                self.handle,
                first_binding,
                buffers,
                offsets,
            )
        }
    }

    pub fn cmd_bind_descriptor_sets(
        &self,
        bind_point: PipelineBindPoint,
        pipeline_layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
    ) {
        unsafe {
            self.device().handle.cmd_bind_descriptor_sets(
                self.handle,
                bind_point,
                pipeline_layout,
                first_set,
                descriptor_sets,
                &[],
            )
        }
    }

    pub fn cmd_draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device().handle.cmd_draw(
                self.handle,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }

    pub fn cmd_image_barrier(
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
            self.device().handle.cmd_pipeline_barrier(
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
