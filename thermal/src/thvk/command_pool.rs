use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, CommandPoolResetFlags,
    },
};

use crate::thvk::{command_buffer::ThCommandBuffer, device::ThDevice};

pub struct ThCommandPool {
    pub handle: CommandPool,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_command_pool(
        self: &Arc<ThDevice>,
        family: u32,
        flags: CommandPoolCreateFlags,
    ) -> VkResult<ThCommandPool> {
        let command_pool_info = CommandPoolCreateInfo {
            flags,
            queue_family_index: family,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_command_pool(&command_pool_info, None) }?;

        Ok(ThCommandPool {
            handle,
            device: self.clone(),
        })
    }
}

impl ThCommandPool {
    pub fn reset(&self) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .reset_command_pool(self.handle, CommandPoolResetFlags::empty())
        }
    }

    pub fn allocate_command_buffer(&self, level: CommandBufferLevel) -> VkResult<ThCommandBuffer> {
        let command_buffer_info = CommandBufferAllocateInfo {
            command_pool: self.handle,
            level,
            command_buffer_count: 1,
            ..Default::default()
        };

        let handle = unsafe {
            self.device
                .handle
                .allocate_command_buffers(&command_buffer_info)
        }?;

        Ok(ThCommandBuffer {
            handle: handle[0],
            device: self.device.clone(),
        })
    }

    pub fn free_command_buffer(&self, command_buffer: CommandBuffer) {
        unsafe {
            self.device
                .handle
                .free_command_buffers(self.handle, &[command_buffer])
        }
    }
}

impl Drop for ThCommandPool {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_command_pool(self.handle, None) }
    }
}
