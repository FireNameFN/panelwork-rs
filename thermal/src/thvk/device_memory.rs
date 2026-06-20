use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        Buffer, DeviceMemory, DeviceSize, Image, MemoryAllocateInfo, MemoryDedicatedAllocateInfo,
        MemoryMapFlags, MemoryPropertyFlags, TaggedStructure,
    },
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::{buffer::ThBuffer, device::ThDevice, handle::ThHandle, image::ThImage};

#[derive(ThDeviceHandle)]
pub struct ThDeviceMemory {
    handle: DeviceMemory,

    device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn allocate_memory(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
    ) -> VkResult<Arc<ThDeviceMemory>> {
        let memory_info = MemoryAllocateInfo {
            allocation_size: size,
            memory_type_index: memory_type,
            ..Default::default()
        };

        let handle = unsafe { self.handle.allocate_memory(&memory_info, None) }?;

        Ok(Arc::new(ThDeviceMemory {
            handle,
            device: self.clone(),
        }))
    }

    pub fn allocate_memory_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
        buffer: Buffer,
    ) -> VkResult<Arc<ThDeviceMemory>> {
        let mut dedicated_info = MemoryDedicatedAllocateInfo {
            buffer,
            ..Default::default()
        };

        let memory_info = MemoryAllocateInfo {
            allocation_size: size,
            memory_type_index: memory_type,
            ..Default::default()
        }
        .push(&mut dedicated_info);

        let handle = unsafe { self.handle.allocate_memory(&memory_info, None) }?;

        Ok(Arc::new(ThDeviceMemory {
            handle,
            device: self.clone(),
        }))
    }

    pub fn allocate_memory_buffer_properties(
        self: &Arc<ThDevice>,
        buffer: &ThBuffer,
        properties: MemoryPropertyFlags,
    ) -> VkResult<Arc<ThDeviceMemory>> {
        let requirements = buffer.memory_requirements();

        let memory_type = self
            .physical_device
            .find_memory_type(requirements.memory_type_bits, properties)
            .unwrap();

        self.allocate_memory_buffer(requirements.size, memory_type, buffer.handle())
    }

    pub fn allocate_memory_image(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
        image: Image,
    ) -> VkResult<Arc<ThDeviceMemory>> {
        let mut dedicated_info = MemoryDedicatedAllocateInfo {
            image,
            ..Default::default()
        };

        let memory_info = MemoryAllocateInfo {
            allocation_size: size,
            memory_type_index: memory_type,
            ..Default::default()
        }
        .push(&mut dedicated_info);

        let handle = unsafe { self.handle.allocate_memory(&memory_info, None) }?;

        Ok(Arc::new(ThDeviceMemory {
            handle,
            device: self.clone(),
        }))
    }

    pub fn allocate_memory_image_properties(
        self: &Arc<ThDevice>,
        image: &ThImage,
        properties: MemoryPropertyFlags,
    ) -> VkResult<Arc<ThDeviceMemory>> {
        let requirements = image.memory_requirements();

        let memory_type = self
            .physical_device
            .find_memory_type(requirements.memory_type_bits, properties)
            .unwrap();

        self.allocate_memory_image(requirements.size, memory_type, image.handle())
    }
}

impl ThDeviceMemory {
    pub fn copy_from<T>(&self, slice: &[T]) -> VkResult<()> {
        if slice.is_empty() {
            return Ok(());
        }

        let size = std::mem::size_of_val(slice);

        let mapping = unsafe {
            self.device.handle.map_memory(
                self.handle,
                0,
                size as DeviceSize,
                MemoryMapFlags::empty(),
            )
        }?;

        unsafe { mapping.copy_from(slice.as_ptr().cast(), size) };

        Ok(())
    }
}

impl Drop for ThDeviceMemory {
    fn drop(&mut self) {
        unsafe { self.device.handle.free_memory(self.handle, None) }
    }
}
