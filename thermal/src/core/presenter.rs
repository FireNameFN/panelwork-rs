use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Extent2D, Fence, Format, ImageUsageFlags, PresentModeKHR, SurfaceKHR},
};

use crate::thvk::{
    handle::ThHandle, image::ThImage, physical_device::ThPhysicalDevice, queue::ThQueue,
    semaphore::ThSemaphore, swapchain::ThSwapchain,
};

pub struct Presenter {
    pub physical_device: ThPhysicalDevice,

    pub queue: ThQueue,

    pub surface: Arc<dyn ThHandle<SurfaceKHR>>,

    pub semaphore: ThSemaphore,

    pub images: Vec<Arc<ThImage>>,

    pub present_semaphores: Vec<ThSemaphore>,

    pub width: u32,

    pub height: u32,

    pub format: Format,

    pub usage: ImageUsageFlags,

    pub present_mode: PresentModeKHR,

    swapchain: Option<ThSwapchain>,
}

impl Presenter {
    pub fn new(
        physical_device: &ThPhysicalDevice,
        queue: &ThQueue,
        surface: Arc<dyn ThHandle<SurfaceKHR>>,
    ) -> VkResult<Self> {
        let semaphore = queue.device.create_semaphore()?;

        Ok(Self {
            physical_device: physical_device.clone(),
            queue: queue.clone(),
            surface,
            semaphore: semaphore,
            images: vec![],
            present_semaphores: vec![],
            width: 0,
            height: 0,
            format: Format::B8G8R8A8_SRGB,
            usage: ImageUsageFlags::empty(),
            present_mode: PresentModeKHR::IMMEDIATE,
            swapchain: None,
        })
    }

    pub fn acquire_next_image(&self, timeout: u64) -> VkResult<(u32, bool)> {
        self.swapchain.as_ref().unwrap().acquire_next_image(
            timeout,
            self.semaphore.handle,
            Fence::null(),
        )
    }

    pub fn present(&self, index: u32) -> VkResult<bool> {
        self.swapchain.as_ref().unwrap().present(
            self.queue.handle,
            &[self.present_semaphores[index as usize].handle],
            index,
        )
    }

    pub fn set_size(&mut self, width: u32, height: u32) -> VkResult<()> {
        let capabilities = self
            .physical_device
            .surface_capabilities(self.surface.handle())?;

        self.width = width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        );

        self.height = height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        );

        let swapchain = self.queue.device.create_swapchain(
            self.surface.handle(),
            capabilities.min_image_count,
            self.format,
            Extent2D {
                width: self.width,
                height: self.height,
            },
            self.usage,
            self.present_mode,
            self.swapchain.as_ref().map(|swapchain| swapchain.handle),
        )?;

        self.queue.wait_idle().unwrap();

        self.images = swapchain.images()?;

        self.present_semaphores = self
            .images
            .iter()
            .map(|_| self.queue.device.create_semaphore())
            .collect::<VkResult<Vec<_>>>()?;

        self.swapchain = Some(swapchain);

        Ok(())
    }
}

impl Drop for Presenter {
    fn drop(&mut self) {
        self.queue.wait_idle().unwrap();
    }
}
