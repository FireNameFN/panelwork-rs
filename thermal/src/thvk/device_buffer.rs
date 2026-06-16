use std::sync::Arc;

use ash::{
    VkResult,
    vk::{BufferUsageFlags, MemoryPropertyFlags},
};

use crate::thvk::{
    buffer::ThBuffer, device::ThDevice, device_memory::ThDeviceMemory,
    physical_device::ThPhysicalDevice,
};

pub struct ThDeviceBuffer {
    buffer: ThBuffer,

    memory: ThDeviceMemory,
}

impl ThDevice {
    pub fn allocate_buffer(
        self: &Arc<ThDevice>,
        physical_device: &ThPhysicalDevice,
        size: u64,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags,
    ) -> VkResult<ThDeviceBuffer> {
        let buffer = self.create_buffer(size, usage)?;

        let memory =
            self.allocate_memory_buffer_properties(physical_device, &buffer, properties)?;

        Ok(ThDeviceBuffer { buffer, memory })
    }
}

impl ThDeviceBuffer {
    pub fn buffer(&self) -> &ThBuffer {
        &self.buffer
    }

    pub fn memory(&self) -> &ThDeviceMemory {
        &self.memory
    }
}
