use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Framebuffer, FramebufferCreateInfo, ImageView},
};

use crate::thvk::render_pass::ThRenderPass;

pub struct ThFramebuffer {
    pub handle: Framebuffer,

    pub render_pass: Arc<ThRenderPass>,
}

impl ThRenderPass {
    pub fn create_framebuffer(
        self: &Arc<ThRenderPass>,
        attachments: &[ImageView],
        width: u32,
        height: u32,
    ) -> VkResult<ThFramebuffer> {
        let framebuffer_info = FramebufferCreateInfo {
            render_pass: self.handle,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width: width,
            height: height,
            layers: 1,
            ..Default::default()
        };

        let handle = unsafe {
            self.device
                .handle
                .create_framebuffer(&framebuffer_info, None)
        }?;

        Ok(ThFramebuffer {
            handle,
            render_pass: self.clone(),
        })
    }
}

impl Drop for ThFramebuffer {
    fn drop(&mut self) {
        unsafe {
            self.render_pass
                .device
                .handle
                .destroy_framebuffer(self.handle, None)
        }
    }
}
