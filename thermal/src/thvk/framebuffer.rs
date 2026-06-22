use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Framebuffer, FramebufferCreateInfo, ImageView, RenderPass},
};

use crate::thvk::{
    device::ThDevice,
    handle::{ThDeviceHandle, ThHandle},
};

#[derive(ThHandle)]
pub struct ThFramebuffer<T: ThDeviceHandle<RenderPass>> {
    handle: Framebuffer,

    render_pass: T,
}

impl<T: ThDeviceHandle<RenderPass>> ThDeviceHandle<Framebuffer> for ThFramebuffer<T> {
    fn device(&self) -> &Arc<ThDevice> {
        self.render_pass.device()
    }
}

pub trait ThRenderPassSource: ThDeviceHandle<RenderPass> + Sized {
    fn create_framebuffer(
        self,
        attachments: &[ImageView],
        width: u32,
        height: u32,
    ) -> VkResult<ThFramebuffer<Self>>;
}

impl<T: ThDeviceHandle<RenderPass>> ThRenderPassSource for T {
    fn create_framebuffer(
        self,
        attachments: &[ImageView],
        width: u32,
        height: u32,
    ) -> VkResult<ThFramebuffer<T>> {
        let framebuffer_info = FramebufferCreateInfo {
            render_pass: self.handle(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width,
            height,
            layers: 1,
            ..Default::default()
        };

        let handle = unsafe {
            self.device()
                .handle
                .create_framebuffer(&framebuffer_info, None)
        }?;

        Ok(ThFramebuffer {
            handle,
            render_pass: self,
        })
    }
}

impl<T: ThDeviceHandle<RenderPass>> ThFramebuffer<T> {
    pub fn render_pass(&self) -> &T {
        &self.render_pass
    }
}

impl<T: ThDeviceHandle<RenderPass>> Drop for ThFramebuffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.render_pass
                .device()
                .handle
                .destroy_framebuffer(self.handle, None)
        }
    }
}
