use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Buffer, BufferCreateInfo, BufferUsageFlags, MemoryPropertyFlags, MemoryRequirements},
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::{device::ThDevice, device_memory::ThDeviceMemory, handle::ThHandle};

#[derive(ThDeviceHandle)]
pub struct ThBuffer {
    handle: Buffer,

    device: Arc<ThDevice>,

    memory: Option<Arc<ThDeviceMemory>>,
}

impl ThDevice {
    pub fn create_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        usage: BufferUsageFlags,
    ) -> VkResult<Arc<ThBuffer>> {
        let buffer_info = BufferCreateInfo {
            size,
            usage,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_buffer(&buffer_info, None) }?;

        Ok(Arc::new(ThBuffer {
            handle,
            device: self.clone(),
            memory: None,
        }))
    }

    pub fn allocate_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags,
    ) -> VkResult<Arc<ThBuffer>> {
        let mut buffer = self.create_buffer(size, usage)?;

        let memory = self.allocate_memory_buffer_properties(&buffer, properties)?;

        Arc::get_mut(&mut buffer).unwrap().bind_memory(memory, 0)?;

        Ok(buffer)
    }
}

impl ThBuffer {
    pub fn memory(&self) -> &Option<Arc<ThDeviceMemory>> {
        &self.memory
    }

    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_buffer_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<ThDeviceMemory>, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_buffer_memory(self.handle, memory.handle(), offset)
        }?;

        self.memory = Some(memory);

        Ok(())
    }
}

impl Drop for ThBuffer {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer(self.handle, None) }
    }
}
