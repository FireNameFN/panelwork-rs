use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        CompositeAlphaFlagsKHR, Extent2D, Fence, Format, ImageUsageFlags, PresentModeKHR,
        Semaphore, SurfaceKHR, SurfaceTransformFlagsKHR, SwapchainCreateInfoKHR, SwapchainKHR,
    },
};

use crate::thvk::{device::ThDevice, image::ThImage};

pub struct ThSwapchain {
    pub handle: SwapchainKHR,

    pub device: Arc<ThDevice>,
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
        old_swapchain: SwapchainKHR,
    ) -> VkResult<ThSwapchain> {
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
            old_swapchain,
            ..Default::default()
        };

        let handle = unsafe {
            self.swapchain_device
                .create_swapchain(&swapchain_info, None)
        }?;

        Ok(ThSwapchain {
            handle,
            device: self.clone(),
        })
    }
}

impl ThSwapchain {
    pub fn images(&self) -> VkResult<Vec<ThImage>> {
        let images = unsafe {
            self.device
                .swapchain_device
                .get_swapchain_images(self.handle)
        }?;

        Ok(images
            .into_iter()
            .map(|image| ThImage {
                handle: image,
                device: self.device.clone(),
                drop: false,
            })
            .collect::<Vec<_>>())
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
