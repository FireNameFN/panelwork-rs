use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        Extent2D, Format, Image, ImageCreateInfo, ImageLayout, ImageType, ImageUsageFlags,
        MemoryPropertyFlags, MemoryRequirements, SampleCountFlags,
    },
};
use thermal_derive::ThDeviceHandle;

use crate::{
    primitives,
    thvk::{device::ThDevice, device_memory::ThDeviceMemory, handle::ThHandle},
};

#[derive(ThDeviceHandle)]
pub struct ThImage {
    handle: Image,

    device: Arc<ThDevice>,

    memory: Option<Arc<ThDeviceMemory>>,
}

impl ThDevice {
    pub fn create_image(
        self: &Arc<ThDevice>,
        format: Format,
        extent: Extent2D,
        mip_levels: u32,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        layout: ImageLayout,
    ) -> VkResult<Arc<ThImage>> {
        let image_info = ImageCreateInfo {
            image_type: ImageType::TYPE_2D,
            format,
            extent: primitives::extent3d(extent.width, extent.height, 1),
            mip_levels,
            array_layers: 1,
            samples,
            usage,
            initial_layout: layout,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_image(&image_info, None) }?;

        Ok(Arc::new(ThImage {
            handle,
            device: self.clone(),
            memory: None,
        }))
    }

    pub fn allocate_image(
        self: &Arc<ThDevice>,
        format: Format,
        extent: Extent2D,
        mip_levels: u32,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
    ) -> VkResult<Arc<ThImage>> {
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

        Arc::get_mut(&mut image).unwrap().bind_memory(memory, 0)?;

        Ok(image)
    }
}

impl ThImage {
    pub fn memory(&self) -> &Option<Arc<ThDeviceMemory>> {
        &self.memory
    }

    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_image_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<ThDeviceMemory>, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_image_memory(self.handle, memory.handle(), offset)
        }?;

        self.memory = Some(memory);

        Ok(())
    }
}

impl Drop for ThImage {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_image(self.handle, None) }
    }
}
