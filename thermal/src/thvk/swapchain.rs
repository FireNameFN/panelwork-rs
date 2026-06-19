use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        CompositeAlphaFlagsKHR, Extent2D, Fence, Format, Image, ImageUsageFlags, PresentInfoKHR,
        PresentModeKHR, Queue, Semaphore, SurfaceKHR, SurfaceTransformFlagsKHR,
        SwapchainCreateInfoKHR, SwapchainKHR,
    },
};
use thermal_derive::ThHandle;

use crate::thvk::{device::ThDevice, handle::ThDeviceHandle};

#[derive(ThDeviceHandle)]
pub struct ThSwapchain {
    handle: SwapchainKHR,

    device: Arc<ThDevice>,
}

#[derive(ThHandle, Clone)]
pub struct ThSwapchainImage {
    handle: Image,

    swapchain: Arc<ThSwapchain>,
}

impl ThDeviceHandle<Image> for ThSwapchainImage {
    fn device(&self) -> &Arc<ThDevice> {
        &self.swapchain.device
    }
}

impl ThDevice {
    pub fn create_swapchain(
        self: &Arc<ThDevice>,
        surface: SurfaceKHR,
        min_image_count: u32,
        format: Format,
        extent: Extent2D,
        usage: ImageUsageFlags,
        present_mode: PresentModeKHR,
        old_swapchain: Option<SwapchainKHR>,
    ) -> VkResult<Arc<ThSwapchain>> {
        let swapchain_info = SwapchainCreateInfoKHR {
            surface,
            min_image_count,
            image_format: format,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: usage,
            pre_transform: SurfaceTransformFlagsKHR::IDENTITY,
            composite_alpha: CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            old_swapchain: old_swapchain.unwrap_or_default(),
            ..Default::default()
        };

        let handle = unsafe {
            self.swapchain_device
                .create_swapchain(&swapchain_info, None)
        }?;

        Ok(Arc::new(ThSwapchain {
            handle,
            device: self.clone(),
        }))
    }
}

impl ThSwapchain {
    pub fn images(self: &Arc<ThSwapchain>) -> VkResult<Vec<ThSwapchainImage>> {
        let images = unsafe {
            self.device
                .swapchain_device
                .get_swapchain_images(self.handle)
        }?;

        Ok(images
            .into_iter()
            .map(|image| ThSwapchainImage {
                handle: image,
                swapchain: self.clone(),
            })
            .collect())
    }

    pub fn acquire_next_image(
        &self,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
    ) -> VkResult<(u32, bool)> {
        unsafe {
            self.device
                .swapchain_device
                .acquire_next_image(self.handle, timeout, semaphore, fence)
        }
    }

    pub fn present(
        &self,
        queue: Queue,
        wait_semaphores: &[Semaphore],
        index: u32,
    ) -> VkResult<bool> {
        let present_info = PresentInfoKHR {
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: &self.handle,
            p_image_indices: &index,
            ..Default::default()
        };

        unsafe {
            self.device
                .swapchain_device
                .queue_present(queue, &present_info)
        }
    }
}

impl Drop for ThSwapchain {
    fn drop(&mut self) {
        unsafe {
            self.device
                .swapchain_device
                .destroy_swapchain(self.handle, None)
        }
    }
}
