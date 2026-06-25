use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        DeviceMemory, Extent2D, Format, Image, ImageCreateInfo, ImageLayout, ImageType,
        ImageUsageFlags, MemoryPropertyFlags, MemoryRequirements, SampleCountFlags,
    },
};

use crate::{
    primitives::vk::extent3d,
    thvk::{
        device::ThDevice, device_memory::ThDeviceMemory, handle::ThDeviceHandle,
        memory_mapping::MemoryMappable,
    },
};

#[derive(ThDeviceHandle)]
pub struct ThImage<T: ThDeviceHandle<DeviceMemory>> {
    handle: Image,

    device: Arc<ThDevice>,

    memory: Option<T>,
}

impl ThDevice {
    pub fn create_image<T: ThDeviceHandle<DeviceMemory>>(
        self: &Arc<ThDevice>,
        format: Format,
        extent: Extent2D,
        mip_levels: u32,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        layout: ImageLayout,
    ) -> VkResult<ThImage<T>> {
        let image_info = ImageCreateInfo {
            image_type: ImageType::TYPE_2D,
            format,
            extent: extent3d(extent.width, extent.height, 1),
            mip_levels,
            array_layers: 1,
            samples,
            usage,
            initial_layout: layout,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_image(&image_info, None) }?;

        Ok(ThImage {
            handle,
            device: self.clone(),
            memory: None,
        })
    }

    pub fn allocate_image(
        self: &Arc<ThDevice>,
        format: Format,
        extent: Extent2D,
        mip_levels: u32,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
    ) -> VkResult<ThImage<ThDeviceMemory>> {
        let mut image = self.create_image(
            format,
            extent,
            mip_levels,
            samples,
            usage,
            ImageLayout::UNDEFINED,
        )?;

        let memory =
            self.allocate_memory_image_properties(&image, MemoryPropertyFlags::DEVICE_LOCAL)?;

        image.bind_memory(memory, 0)?;

        Ok(image)
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> ThImage<T> {
    pub fn memory(&mut self) -> Option<&mut T> {
        self.memory.as_mut()
    }

    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_image_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&mut self, memory: T, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_image_memory(self.handle, memory.handle(), offset)
        }?;

        self.memory = Some(memory);

        Ok(())
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> MemoryMappable for ThImage<T> {
    type Memory = T;

    fn memory(&self) -> &Self::Memory {
        self.memory.as_ref().unwrap()
    }
}

impl<T: ThDeviceHandle<DeviceMemory>> Drop for ThImage<T> {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_image(self.handle, None) }
    }
}
