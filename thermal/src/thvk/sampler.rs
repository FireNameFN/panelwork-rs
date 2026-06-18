use std::sync::Arc;

use ash::{
    VkResult,
    vk::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
};
use thermal_derive::ThDeviceHandle;

use crate::thvk::device::ThDevice;

#[derive(ThDeviceHandle)]
pub struct ThSampler {
    handle: Sampler,

    device: Arc<ThDevice>,
}

impl ThDevice {
    pub fn create_sampler(
        self: &Arc<ThDevice>,
        filter: Filter,
        address_mode: SamplerAddressMode,
    ) -> VkResult<ThSampler> {
        let sampler_info = SamplerCreateInfo {
            mag_filter: filter,
            min_filter: filter,
            address_mode_u: address_mode,
            address_mode_v: address_mode,
            address_mode_w: address_mode,
            ..Default::default()
        };

        let handle = unsafe { self.handle.create_sampler(&sampler_info, None) }?;

        Ok(ThSampler {
            handle: handle,
            device: self.clone(),
        })
    }
}

impl Drop for ThSampler {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_sampler(self.handle, None) }
    }
}
