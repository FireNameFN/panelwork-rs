use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        ComponentMapping, Format, ImageSubresourceRange, ImageView, ImageViewCreateInfo,
        ImageViewType,
    },
};

use crate::thvk::image::ThImage;

pub struct ThImageView {
    pub handle: ImageView,

    pub image: Arc<ThImage>,
}

impl ThImage {
    pub fn create_image_view(
        self: &Arc<ThImage>,
        format: Format,
        mapping: ComponentMapping,
        range: ImageSubresourceRange,
    ) -> VkResult<ThImageView> {
        let image_view_info = ImageViewCreateInfo {
            image: self.handle,
            view_type: ImageViewType::TYPE_2D,
            format,
            components: mapping,
            subresource_range: range,
            ..Default::default()
        };

        let handle = unsafe { self.device.handle.create_image_view(&image_view_info, None) }?;

        Ok(ThImageView {
            handle,
            image: self.clone(),
        })
    }
}

impl Drop for ThImageView {
    fn drop(&mut self) {
        unsafe {
            self.image
                .device
                .handle
                .destroy_image_view(self.handle, None)
        }
    }
}
