use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        Buffer, DeviceMemory, DeviceSize, Image, MemoryAllocateInfo, MemoryDedicatedAllocateInfo,
        MemoryMapFlags, TaggedStructure,
    },
};

use crate::thvk::device::ThDevice;

pub struct ThDeviceMemory {
    pub handle: DeviceMemory,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn allocate_memory(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
    ) -> VkResult<ThDeviceMemory> {
        let memory_info = MemoryAllocateInfo {
            allocation_size: size,
            memory_type_index: memory_type,
            ..Default::default()
        };

        let handle = unsafe { self.handle.allocate_memory(&memory_info, None) }?;

        Ok(ThDeviceMemory {
            handle,
            device: self.clone(),
        })
    }

    pub fn allocate_memory_buffer(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
        buffer: Buffer,
    ) -> VkResult<ThDeviceMemory> {
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

        Ok(ThDeviceMemory {
            handle,
            device: self.clone(),
        })
    }

    pub fn allocate_memory_image(
        self: &Arc<ThDevice>,
        size: u64,
        memory_type: u32,
        image: Image,
    ) -> VkResult<ThDeviceMemory> {
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

        Ok(ThDeviceMemory {
            handle,
            device: self.clone(),
        })
    }
}

impl ThDeviceMemory {
    pub fn copy_from(&self, slice: &[u8]) -> VkResult<()> {
        if slice.is_empty() {
            return Ok(());
        }

        let mapping = unsafe {
            self.device.handle.map_memory(
                self.handle,
                0,
                slice.len() as DeviceSize,
                MemoryMapFlags::empty(),
            )
        }?;

        unsafe { mapping.copy_from(slice.as_ptr().cast(), slice.len()) };

        Ok(())
    }
}

impl Drop for ThDeviceMemory {
    fn drop(&mut self) {
        unsafe { self.device.handle.free_memory(self.handle, None) }
    }
}
