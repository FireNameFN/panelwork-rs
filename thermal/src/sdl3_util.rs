use std::ffi::CStr;

use ash::vk::{Handle, Instance, PhysicalDevice};

use crate::util;

pub fn sdl_instance_extensions<'a>() -> Vec<&'a CStr> {
    unsafe {
        util::string_array_from_fn(|size| {
            let ptr = sdl3_sys::vulkan::SDL_Vulkan_GetInstanceExtensions(size);

            if ptr.is_null() {
                panic!("sdl is not initialized")
            }

            ptr
        })
    }
}

pub fn sdl_presentation_support(
    instance: Instance,
    physical_device: PhysicalDevice,
    family: u32,
) -> bool {
    unsafe {
        sdl3_sys::vulkan::SDL_Vulkan_GetPresentationSupport(
            instance.as_raw() as _,
            physical_device.as_raw() as _,
            family,
        )
    }
}
