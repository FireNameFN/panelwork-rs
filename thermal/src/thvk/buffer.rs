use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        Buffer, BufferCreateInfo, BufferUsageFlags, DeviceMemory, MemoryPropertyFlags,
        MemoryRequirements,
    },
};

use crate::thvk::{
    device::ThDevice, device_memory::ThDeviceMemory, handle::ThDeviceHandle,
    memory_mapping::MemoryMappable,
};

#[derive(ThDeviceHandle)]
pub struct ThBuffer<T: ThDeviceHandle<DeviceMemory>> {
    handle: Buffer,

    device: Arc<ThDevice>,

    memory: Option<T>,
}

impl ThDevice {
    pub fn create_buffer<T: ThDeviceHandle<DeviceMemory>>(
        self: &Arc<ThDevice>,
        size: u64,
        usage: BufferUsageFlags,
    ) -> VkResult<ThBuffer<T>> {
        let buffer_info = BufferCreateInfo {
            size,
            usage,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_buffer(&buffer_info, None) }?;

        Ok(ThBuffer {
            handle,
            device: self.clone(),
            memory: None,
        })
    }

    pub fn allocate_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags,
    ) -> VkResult<ThBuffer<ThDeviceMemory>> {
        let mut buffer = self.create_buffer(size, usage)?;

        let memory = self.allocate_memory_buffer_properties(&buffer, properties)?;

        buffer.bind_memory(memory, 0)?;

        Ok(buffer)
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> ThBuffer<T> {
    pub fn memory(&mut self) -> Option<&mut T> {
        self.memory.as_mut()
    }

    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_buffer_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&mut self, memory: T, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_buffer_memory(self.handle, memory.handle(), offset)
        }?;

        self.memory = Some(memory);

        Ok(())
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> MemoryMappable for ThBuffer<T> {
    type Memory = T;

    fn memory(&self) -> &Self::Memory {
        self.memory.as_ref().unwrap()
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> Drop for ThBuffer<T> {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_buffer(self.handle, None) }
    }
}
