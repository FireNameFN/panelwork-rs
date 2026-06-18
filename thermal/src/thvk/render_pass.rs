use std::sync::Arc;

use ash::{
    VkResult,
    vk::{AttachmentDescription, RenderPass, RenderPassCreateInfo, SubpassDescription},
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::device::ThDevice;

#[derive(ThDeviceHandle)]
pub struct ThRenderPass {
    handle: RenderPass,

    device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_render_pass(
        self: &Arc<ThDevice>,
        attachments: &[AttachmentDescription],
        subpasses: &[SubpassDescription],
    ) -> VkResult<Arc<ThRenderPass>> {
        let render_pass_info = RenderPassCreateInfo {
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_render_pass(&render_pass_info, None) }?;

        Ok(Arc::new(ThRenderPass {
            handle,
            device: self.clone(),
        }))
    }
}

impl Drop for ThRenderPass {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_render_pass(self.handle, None) }
    }
}
