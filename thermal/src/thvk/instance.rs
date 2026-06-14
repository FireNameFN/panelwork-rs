use std::{ffi::CStr, sync::Arc};

use ash::{
    Instance, VkResult,
    khr::surface,
    vk::{self, ApplicationInfo, InstanceCreateInfo},
};

use crate::{
    ext::physical_device::ThPhysicalDeviceIteratorExt,
    thvk::{library::ThLibrary, physical_device::ThPhysicalDevice},
    util,
};

pub struct ThInstance {
    pub handle: Instance,

    pub library: Arc<ThLibrary>,

    pub surface_instance: surface::Instance,
}

impl ThLibrary {
    pub fn create_instance(
        self: &Arc<ThLibrary>,
        version: u32,
        layers: &[&CStr],
        extensions: &[&CStr],
    ) -> VkResult<Arc<ThInstance>> {
        let application_info = ApplicationInfo {
            p_engine_name: c"Thermal".as_ptr(),
            engine_version: vk::make_api_version(0, 0, 0, 0),
            api_version: version,
            ..Default::default()
        };

        let layers_ptr = util::string_array_to_ptr(layers);

        let extensions_ptr = util::string_array_to_ptr(extensions);

        let instance_info = InstanceCreateInfo {
            p_application_info: &application_info,
            enabled_layer_count: layers_ptr.len() as u32,
            pp_enabled_layer_names: layers_ptr.as_ptr(),
            enabled_extension_count: extensions_ptr.len() as u32,
            pp_enabled_extension_names: extensions_ptr.as_ptr(),
            ..Default::default()
        };

        let handle = unsafe { self.entry.create_instance(&instance_info, None) }?;

        let surface_instance = surface::Instance::load(&self.entry, &handle);

        Ok(Arc::new(ThInstance {
            handle,
            surface_instance: surface_instance,
            library: self.clone(),
        }))
    }
}

impl ThInstance {
    pub fn physical_devices(self: &Arc<Self>) -> VkResult<impl ThPhysicalDeviceIteratorExt> {
        let physical_devices = unsafe { self.handle.enumerate_physical_devices() }?;

        Ok(physical_devices.into_iter().map(|device| ThPhysicalDevice {
            handle: device,
            instance: self.clone(),
        }))
    }
}

impl Drop for ThInstance {
    fn drop(&mut self) {
        unsafe { self.handle.destroy_instance(None) }
    }
}
