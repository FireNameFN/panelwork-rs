use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        ComponentMapping, Format, Image, ImageSubresourceRange, ImageView, ImageViewCreateInfo,
        ImageViewType,
    },
};
use thermal_derive::ThHandle;

use crate::thvk::{device::ThDevice, handle::ThDeviceHandle};

#[derive(ThHandle)]
pub struct ThImageView<T: ThDeviceHandle<Image>> {
    handle: ImageView,

    image: T,
}

impl<T: ThDeviceHandle<Image>> ThDeviceHandle<ImageView> for ThImageView<T> {
    fn device(&self) -> &Arc<ThDevice> {
        self.image.device()
    }
}

pub trait ThImageViewSource: ThDeviceHandle<Image> + Sized {
    fn create_image_view(
        self,
        format: Format,
        mapping: ComponentMapping,
        range: ImageSubresourceRange,
    ) -> VkResult<ThImageView<Self>>;
}

impl<T: ThDeviceHandle<Image>> ThImageViewSource for T {
    fn create_image_view(
        self,
        format: Format,
        mapping: ComponentMapping,
        range: ImageSubresourceRange,
    ) -> VkResult<ThImageView<T>> {
        let image_view_info = ImageViewCreateInfo {
            image: self.handle(),
            view_type: ImageViewType::TYPE_2D,
            format,
            components: mapping,
            subresource_range: range,
            ..Default::default()
        };

        let handle = unsafe {
            self.device()
                .handle
                .create_image_view(&image_view_info, None)
        }?;

        Ok(ThImageView {
            handle,
            image: self,
        })
    }
}

impl<T: ThDeviceHandle<Image>> ThImageView<T> {
    pub fn image(&self) -> &T {
        &self.image
    }
}

impl<T: ThDeviceHandle<Image>> Drop for ThImageView<T> {
    fn drop(&mut self) {
        unsafe {
            self.image
                .device()
                .handle
                .destroy_image_view(self.handle, None)
        }
    }
}
