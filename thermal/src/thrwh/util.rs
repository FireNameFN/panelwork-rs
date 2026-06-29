use std::ffi::CStr;

use ash::VkResult;
use raw_window_handle::RawDisplayHandle;

pub fn rwh_instance_extensions(display_handle: RawDisplayHandle) -> VkResult<Vec<&'static CStr>> {
    Ok(ash_window::enumerate_required_extensions(display_handle)?
        .iter()
        .map(|ptr| unsafe { CStr::from_ptr(*ptr) })
        .collect())
}
