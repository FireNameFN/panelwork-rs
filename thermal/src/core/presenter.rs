use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Fence, Format, ImageUsageFlags, PresentModeKHR, SurfaceKHR},
};

use crate::{
    primitives::vk::extent,
    thvk::{
        handle::{ThDeviceHandle, ThHandle},
        queue::ThQueue,
        semaphore::ThSemaphore,
        swapchain::{ThSwapchain, ThSwapchainImage},
    },
};

pub struct Presenter<T: ThHandle<SurfaceKHR>> {
    queue: ThQueue,

    surface: T,

    semaphore: ThSemaphore,

    images: Vec<ThSwapchainImage>,

    present_semaphores: Vec<ThSemaphore>,

    width: u32,

    height: u32,

    pub format: Format,

    pub usage: ImageUsageFlags,

    pub present_mode: PresentModeKHR,

    swapchain: Option<Arc<ThSwapchain>>,
}

impl<T: ThHandle<SurfaceKHR>> Presenter<T> {
    pub fn new(queue: ThQueue, surface: T) -> VkResult<Self> {
        let semaphore = queue.device().create_semaphore()?;

        Ok(Self {
            queue,
            surface,
            semaphore,
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

    pub fn queue(&self) -> &ThQueue {
        &self.queue
    }

    pub fn surface(&self) -> &T {
        &self.surface
    }

    pub fn semaphore(&self) -> &ThSemaphore {
        &self.semaphore
    }

    pub fn images(&self) -> &Vec<ThSwapchainImage> {
        &self.images
    }

    pub fn present_semaphores(&self) -> &Vec<ThSemaphore> {
        &self.present_semaphores
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn acquire_next_image(&self, timeout: u64) -> VkResult<(u32, bool)> {
        self.swapchain.as_ref().unwrap().acquire_next_image(
            timeout,
            self.semaphore.handle(),
            Fence::null(),
        )
    }

    pub fn present(&self, index: u32) -> VkResult<bool> {
        self.swapchain.as_ref().unwrap().present(
            self.queue.handle(),
            &[self.present_semaphores[index as usize].handle()],
            index,
        )
    }

    pub fn set_size(&mut self, width: u32, height: u32) -> VkResult<()> {
        let capabilities = self
            .queue
            .device()
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

        let swapchain = self.queue.device().create_swapchain(
            self.surface.handle(),
            capabilities.min_image_count,
            self.format,
            extent(self.width, self.height),
            self.usage,
            self.present_mode,
            self.swapchain.as_ref().map(|swapchain| swapchain.handle()),
        )?;

        self.queue.wait_idle().unwrap();

        self.images = swapchain.images()?;

        self.present_semaphores.reserve(self.images.len());

        for _ in self.present_semaphores.len()..self.images.len() {
            let semaphore = self.queue.device().create_semaphore()?;

            self.present_semaphores.push(semaphore);
        }

        self.swapchain = Some(swapchain);

        Ok(())
    }
}

impl<T: ThHandle<SurfaceKHR>> Drop for Presenter<T> {
    fn drop(&mut self) {
        self.queue.wait_idle().unwrap();
    }
}
