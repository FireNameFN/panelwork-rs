use std::{ffi::CStr, slice};

pub fn string_array_to_ptr(string_array: &[&CStr]) -> Vec<*const i8> {
    string_array.iter().map(|str| str.as_ptr().cast()).collect()
}

pub unsafe fn string_array_from_ptr<'a>(ptr: *const *const i8, size: usize) -> Vec<&'a CStr> {
    unsafe {
        slice::from_raw_parts(ptr, size)
            .iter()
            .map(|ptr| CStr::from_ptr(*ptr))
            .collect()
    }
}

pub unsafe fn string_array_from_fn<'a>(
    function: impl FnOnce(&mut u32) -> *const *const i8,
) -> Vec<&'a CStr> {
    let mut size = 0;

    let ptr = function(&mut size);

    unsafe { string_array_from_ptr(ptr, size as usize) }
}
