use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        DeviceMemory, Extent2D, Extent3D, Format, Image, ImageCreateInfo, ImageLayout, ImageType,
        ImageUsageFlags, MemoryRequirements, SampleCountFlags,
    },
};

use crate::thvk::device::ThDevice;

pub struct ThImage {
    pub handle: Image,

    pub device: Arc<ThDevice>,

    pub drop: bool,
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
            extent: Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            },
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
            drop: true,
        }))
    }
}

impl ThImage {
    pub fn memory_requirements(&self) -> MemoryRequirements {
        unsafe {
            self.device
                .handle
                .get_image_memory_requirements(self.handle)
        }
    }

    pub fn bind_memory(&self, memory: DeviceMemory, offset: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .bind_image_memory(self.handle, memory, offset)
        }
    }
}

impl Drop for ThImage {
    fn drop(&mut self) {
        if self.drop {
            unsafe { self.device.handle.destroy_image(self.handle, None) }
        }
    }
}
