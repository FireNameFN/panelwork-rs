use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, CommandPoolResetFlags,
    },
};

use crate::thvk::{command_buffer::ThCommandBuffer, queue::ThQueue};

pub struct ThCommandPool {
    pub handle: CommandPool,

    pub queue: ThQueue,
}

impl ThQueue {
    pub fn create_command_pool(
        self: &ThQueue,
        flags: CommandPoolCreateFlags,
    ) -> VkResult<Arc<ThCommandPool>> {
        let command_pool_info = CommandPoolCreateInfo {
            flags,
            queue_family_index: self.family,
            ..Default::default()
        };

        let handle = unsafe {
            self.device
                .handle
                .create_command_pool(&command_pool_info, None)
        }?;

        Ok(Arc::new(ThCommandPool {
            handle,
            queue: self.clone(),
        }))
    }
}

impl ThCommandPool {
    pub fn allocate_command_buffer(
        self: &Arc<ThCommandPool>,
        level: CommandBufferLevel,
    ) -> VkResult<ThCommandBuffer> {
        let command_buffer_info = CommandBufferAllocateInfo {
            command_pool: self.handle,
            level,
            command_buffer_count: 1,
            ..Default::default()
        };

        let handle = unsafe {
            self.queue
                .device
                .handle
                .allocate_command_buffers(&command_buffer_info)
        }?;

        Ok(ThCommandBuffer {
            handle: handle[0],
            command_pool: self.clone(),
        })
    }

    pub fn reset(&self) -> VkResult<()> {
        unsafe {
            self.queue
                .device
                .handle
                .reset_command_pool(self.handle, CommandPoolResetFlags::empty())
        }
    }

    pub fn free_command_buffer(&self, command_buffer: CommandBuffer) {
        unsafe {
            self.queue
                .device
                .handle
                .free_command_buffers(self.handle, &[command_buffer])
        }
    }
}

impl Drop for ThCommandPool {
    fn drop(&mut self) {
        unsafe {
            self.queue
                .device
                .handle
                .destroy_command_pool(self.handle, None)
        }
    }
}
