use std::{ffi::c_void, sync::Arc};

use ash::{
    VkResult,
    vk::{
        Buffer, DeviceMemory, DeviceSize, Image, MemoryAllocateInfo, MemoryDedicatedAllocateInfo,
        MemoryMapFlags, MemoryPropertyFlags, TaggedStructure,
    },
};

use crate::thvk::{
    buffer::ThBuffer,
    device::ThDevice,
    handle::{ThDeviceHandle, ThHandle},
    image::ThImage,
};

#[derive(ThDeviceHandle)]
pub struct ThDeviceMemory {
    handle: DeviceMemory,

    device: Arc<ThDevice>,

    mapping: Option<*mut c_void>,
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
            mapping: None,
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
            mapping: None,
        })
    }

    pub fn allocate_memory_buffer_properties<T: ThDeviceHandle<DeviceMemory>>(
        self: &Arc<ThDevice>,
        buffer: &ThBuffer<T>,
        properties: MemoryPropertyFlags,
    ) -> VkResult<ThDeviceMemory> {
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
            mapping: None,
        })
    }

    pub fn allocate_memory_image_properties<T: ThDeviceHandle<DeviceMemory>>(
        self: &Arc<ThDevice>,
        image: &ThImage<T>,
        properties: MemoryPropertyFlags,
    ) -> VkResult<ThDeviceMemory> {
        let requirements = image.memory_requirements();

        let memory_type = self
            .physical_device
            .find_memory_type(requirements.memory_type_bits, properties)
            .unwrap();

        self.allocate_memory_image(requirements.size, memory_type, image.handle())
    }
}

impl ThDeviceMemory {
    pub fn map(&mut self) -> VkResult<()> {
        self.mapping = Some(unsafe {
            self.device.handle.map_memory(
                self.handle,
                0,
                ash::vk::WHOLE_SIZE,
                MemoryMapFlags::empty(),
            )
        }?);

        Ok(())
    }

    pub fn unmap(&mut self) {
        unsafe { self.device.handle.unmap_memory(self.handle) };

        self.mapping = None;
    }

    pub fn copy_from_mapped(&self, slice: &[impl Clone]) {
        unsafe {
            self.mapping
                .unwrap()
                .copy_from_nonoverlapping(slice.as_ptr().cast(), std::mem::size_of_val(slice))
        };
    }

    pub fn copy_from_unmapped(&self, slice: &[impl Clone]) -> VkResult<()> {
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

        unsafe { mapping.copy_from_nonoverlapping(slice.as_ptr().cast(), size) };

        unsafe { self.device.handle.unmap_memory(self.handle) };

        Ok(())
    }
}

impl Drop for ThDeviceMemory {
    fn drop(&mut self) {
        unsafe { self.device.handle.free_memory(self.handle, None) }
    }
}
