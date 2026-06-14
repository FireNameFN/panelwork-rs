use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        Extent2D, Fence, Format, ImageUsageFlags, PresentInfoKHR, PresentModeKHR, SurfaceKHR,
        SwapchainKHR,
    },
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

    pub images: Vec<ThImage>,

    pub present_semaphores: Vec<ThSemaphore>,

    pub width: i32,

    pub height: i32,

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
        let present_info = PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: &self.present_semaphores[index as usize].handle,
            swapchain_count: 1,
            p_swapchains: &self.swapchain.as_ref().unwrap().handle,
            p_image_indices: &index,
            ..Default::default()
        };

        unsafe {
            self.queue
                .device
                .swapchain_device
                .queue_present(self.queue.handle, &present_info)
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) -> VkResult<()> {
        let capabilities = self
            .physical_device
            .surface_capabilities(self.surface.handle())?;

        self.width = width.clamp(
            capabilities.min_image_extent.width as i32,
            capabilities.max_image_extent.width as i32,
        );

        self.height = height.clamp(
            capabilities.min_image_extent.height as i32,
            capabilities.max_image_extent.height as i32,
        );

        let old_swapchain = match self.swapchain.as_ref() {
            None => SwapchainKHR::null(),
            Some(swapchain) => swapchain.handle,
        };

        let swapchain = self.queue.device.create_swapchain(
            self.surface.handle(),
            capabilities.min_image_count,
            self.format,
            Extent2D {
                width: self.width as u32,
                height: self.height as u32,
            },
            self.usage,
            self.present_mode,
            old_swapchain,
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
