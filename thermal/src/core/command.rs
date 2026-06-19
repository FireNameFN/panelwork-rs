use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        AccessFlags, BufferImageCopy, BufferUsageFlags, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, Format, ImageLayout, ImageUsageFlags,
        MemoryPropertyFlags, PipelineStageFlags, SampleCountFlags,
    },
};

use crate::{
    defaults, primitives,
    thvk::{
        command_buffer::ThCommandBuffer,
        fence::ThFence,
        handle::{ThDeviceHandle, ThHandle},
        image::ThImage,
        queue::ThQueue,
    },
};

pub struct Command {
    command_buffer: ThCommandBuffer,

    fence: ThFence,
}

impl Command {
    pub fn new(queue: ThQueue) -> VkResult<Self> {
        let command_pool = queue.create_command_pool(CommandPoolCreateFlags::TRANSIENT)?;

        let command_buffer = command_pool.allocate_command_buffer(CommandBufferLevel::PRIMARY)?;

        let fence = queue.device().create_fence()?;

        Ok(Self {
            command_buffer,
            fence,
        })
    }

    pub fn execute(&self) -> VkResult<()> {
        self.command_buffer.command_pool.queue().submit(
            self.fence.handle(),
            &[],
            &[],
            &[self.command_buffer.handle],
            &[],
        )?;

        self.fence.wait(u64::MAX)?;

        self.fence.reset()?;

        self.command_buffer.command_pool.reset()?;

        Ok(())
    }

    pub fn create_texture(
        &self,
        format: Format,
        mip_levels: u32,
        slice: &[u8],
        width: u32,
        height: u32,
        pixel_size: u32,
    ) -> VkResult<Arc<ThImage>> {
        let image = self.command_buffer.device().allocate_image(
            format,
            primitives::extent(width, height),
            mip_levels,
            SampleCountFlags::TYPE_1,
            ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED,
        )?;

        let size = width as u64 * height as u64 * pixel_size as u64;

        let buffer = self.command_buffer.device().allocate_buffer(
            size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )?;

        buffer.memory().as_ref().unwrap().copy_from(slice)?;

        let image_copy = BufferImageCopy {
            buffer_row_length: width,
            buffer_image_height: height,
            image_subresource: defaults::SUBRESOURCE_COLOR_LAYER,
            image_extent: primitives::extent3d(width, height, 1),
            ..Default::default()
        };

        self.command_buffer
            .begin(CommandBufferUsageFlags::ONE_TIME_SUBMIT)?;

        self.command_buffer.cmd_image_barrier(
            image.handle(),
            AccessFlags::NONE,
            AccessFlags::TRANSFER_WRITE,
            ImageLayout::UNDEFINED,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlags::TRANSFER,
        );

        unsafe {
            self.command_buffer
                .device()
                .handle
                .cmd_copy_buffer_to_image(
                    self.command_buffer.handle,
                    buffer.handle(),
                    image.handle(),
                    ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[image_copy],
                )
        };

        self.command_buffer.cmd_image_barrier(
            image.handle(),
            AccessFlags::TRANSFER_WRITE,
            AccessFlags::SHADER_READ,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::FRAGMENT_SHADER,
        );

        self.command_buffer.end()?;

        self.execute()?;

        Ok(image)
    }
}
