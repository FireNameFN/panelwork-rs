use ash::{
    VkResult,
    vk::{
        ComponentMapping, Format, Image, ImageSubresourceRange, ImageView, ImageViewCreateInfo,
        ImageViewType,
    },
};

use crate::thvk::handle::ThHandleSource;

pub struct ThImageView<T: ThHandleSource<Image>> {
    pub handle: ImageView,

    pub image: T,
}

pub trait ThImageViewSource: ThHandleSource<Image> {
    fn create_image_view(
        &self,
        format: Format,
        mapping: ComponentMapping,
        range: ImageSubresourceRange,
    ) -> VkResult<ThImageView<Self>>;
}

impl<T: ThHandleSource<Image>> ThImageViewSource for T {
    fn create_image_view(
        &self,
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
            image: self.clone(),
        })
    }
}

impl<T: ThHandleSource<Image>> Drop for ThImageView<T> {
    fn drop(&mut self) {
        unsafe {
            self.image
                .device()
                .handle
                .destroy_image_view(self.handle, None)
        }
    }
}
