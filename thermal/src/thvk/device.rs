use std::{ffi::CStr, sync::Arc};

use ash::{
    Device, VkResult,
    khr::swapchain,
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo},
};

use crate::{thvk::physical_device::ThPhysicalDevice, util};

pub struct ThDevice {
    pub handle: Device,

    pub physical_device: ThPhysicalDevice,

    pub swapchain_device: swapchain::Device,
}

pub struct QueueInfo<'a> {
    pub index: u32,
    pub priorities: &'a [f32],
}

impl ThPhysicalDevice {
    pub fn create_device(
        &self,
        queue_info_slice: &[QueueInfo],
        extensions: &[&CStr],
    ) -> VkResult<Arc<ThDevice>> {
        let queue_info = queue_info_slice
            .iter()
            .map(|queue| DeviceQueueCreateInfo {
                queue_family_index: queue.index,
                queue_count: queue.priorities.len() as u32,
                p_queue_priorities: queue.priorities.as_ptr(),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let extensions_ptr = util::string_array_to_ptr(extensions);

        let device_info = DeviceCreateInfo {
            queue_create_info_count: queue_info.len() as u32,
            p_queue_create_infos: queue_info.as_ptr(),
            enabled_extension_count: extensions_ptr.len() as u32,
            pp_enabled_extension_names: extensions_ptr.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe {
            self.instance
                .handle
                .create_device(self.handle, &device_info, None)
        }?;

        let swapchain_device = swapchain::Device::load(&self.instance.handle, &handle);

        Ok(Arc::new(ThDevice {
            handle,
            physical_device: self.clone(),
            swapchain_device,
        }))
    }
}

impl Drop for ThDevice {
    fn drop(&mut self) {
        unsafe { self.handle.destroy_device(None) }
    }
}
