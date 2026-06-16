use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Buffer, BufferCreateInfo, BufferUsageFlags, DeviceMemory, MemoryRequirements},
};

use crate::thvk::device::ThDevice;

pub struct ThBuffer {
    pub handle: Buffer,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        usage: BufferUsageFlags,
    ) -> VkResult<ThBuffer> {
        let buffer_info = BufferCreateInfo {
            size,
            usage,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_buffer(&buffer_info, None) }?;

        Ok(ThBuffer {
            handle,
            device: self.clone(),
        })
    }
}

impl ThBuffer {
    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_buffer_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&self, memory: DeviceMemory, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_buffer_memory(self.handle, memory, offset)
        }
    }
}

impl Drop for ThBuffer {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer(self.handle, None) }
    }
}
