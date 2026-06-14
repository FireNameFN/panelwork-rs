use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Fence, FenceCreateInfo},
};

use crate::thvk::device::ThDevice;

pub struct ThFence {
    pub handle: Fence,

    pub device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_fence(self: &Arc<ThDevice>) -> VkResult<ThFence> {
        let handle = unsafe { self.handle.create_fence(&FenceCreateInfo::default(), None) }?;

        Ok(ThFence {
            handle,
            device: self.clone(),
        })
    }
}

impl ThFence {
    pub fn wait(&self, timeout: u64) -> VkResult<()> {
        unsafe {
            self.device
                .handle
                .wait_for_fences(&[self.handle], true, timeout)
        }
    }

    pub fn reset(&self) -> VkResult<()> {
        unsafe { self.device.handle.reset_fences(&[self.handle]) }
    }
}

impl Drop for ThFence {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_fence(self.handle, None) }
    }
}
