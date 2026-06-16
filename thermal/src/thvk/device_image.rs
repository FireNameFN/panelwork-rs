use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Extent2D, Format, ImageLayout, ImageUsageFlags, MemoryPropertyFlags, SampleCountFlags},
};

use crate::thvk::{
    device::ThDevice, device_memory::ThDeviceMemory, image::ThImage,
    physical_device::ThPhysicalDevice,
};

pub struct ThDeviceImage {
    image: Arc<ThImage>,

    memory: ThDeviceMemory,
}

impl ThDevice {
    pub fn allocate_image(
        self: &Arc<ThDevice>,
        physical_device: &ThPhysicalDevice,
        format: Format,
        extent: Extent2D,
        mip_levels: u32,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
    ) -> VkResult<ThDeviceImage> {
        let image = self.create_image(
            format,
            extent,
            mip_levels,
            samples,
            usage,
            ImageLayout::UNDEFINED,
        )?;

        let memory = self.allocate_memory_image_properties(
            physical_device,
            &image,
            MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        image.bind_memory(memory.handle, 0)?;

        Ok(ThDeviceImage { image, memory })
    }
}

impl ThDeviceImage {
    pub fn image(&self) -> &Arc<ThImage> {
        &self.image
    }

    pub fn memory(&self) -> &ThDeviceMemory {
        &self.memory
    }
}
