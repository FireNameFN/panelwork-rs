use std::ffi::CStr;

use crate::util;

pub fn sdl_instance_extensions<'a>() -> Vec<&'a CStr> {
    unsafe {
        util::string_array_from_fn(|size| sdl3_sys::vulkan::SDL_Vulkan_GetInstanceExtensions(size))
    }
}
