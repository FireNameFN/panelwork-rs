use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Semaphore, SemaphoreCreateInfo},
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::device::ThDevice;

#[derive(ThDeviceHandle)]
pub struct ThSemaphore {
    handle: Semaphore,

    device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_semaphore(self: &Arc<ThDevice>) -> VkResult<ThSemaphore> {
        let handle = unsafe {
            self.handle
                .create_semaphore(&SemaphoreCreateInfo::default(), None)
        }?;

        Ok(ThSemaphore {
            handle,
            device: self.clone(),
        })
    }
}

impl Drop for ThSemaphore {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_semaphore(self.handle, None) }
    }
}
