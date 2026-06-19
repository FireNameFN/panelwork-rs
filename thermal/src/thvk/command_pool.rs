use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, CommandPoolResetFlags,
    },
};
use thermal_derive::ThHandle;

use crate::thvk::{
    command_buffer::ThCommandBuffer, device::ThDevice, handle::ThDeviceHandle, queue::ThQueue,
};

#[derive(ThHandle)]
pub struct ThCommandPool {
    handle: CommandPool,

    queue: ThQueue,
}

impl ThDeviceHandle<CommandPool> for ThCommandPool {
    fn device(&self) -> &Arc<ThDevice> {
        self.queue.device()
    }
}

impl ThQueue {
    pub fn create_command_pool(
        self: &ThQueue,
        flags: CommandPoolCreateFlags,
    ) -> VkResult<Arc<ThCommandPool>> {
        let command_pool_info = CommandPoolCreateInfo {
            flags,
            queue_family_index: self.family(),
            ..Default::default()
        };

        let handle = unsafe {
            self.device()
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
    pub fn queue(&self) -> &ThQueue {
        &self.queue
    }

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
            self.device()
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
            self.device()
                .handle
                .reset_command_pool(self.handle, CommandPoolResetFlags::empty())
        }
    }

    pub fn free_command_buffer(&self, command_buffer: CommandBuffer) {
        unsafe {
            self.device()
                .handle
                .free_command_buffers(self.handle, &[command_buffer])
        }
    }
}

impl Drop for ThCommandPool {
    fn drop(&mut self) {
        unsafe { self.device().handle.destroy_command_pool(self.handle, None) }
    }
}
