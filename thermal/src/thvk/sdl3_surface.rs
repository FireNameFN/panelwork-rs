use std::{ptr, sync::Arc};

use ash::vk::{Handle, SurfaceKHR};
use sdl3_sys::video::SDL_Window;

use crate::thvk::{handle::ThHandle, instance::ThInstance};

pub struct ThSdl3Surface {
    pub handle: SurfaceKHR,

    pub instance: Arc<ThInstance>,
}

impl ThHandle<SurfaceKHR> for ThSdl3Surface {
    fn handle(&self) -> SurfaceKHR {
        self.handle
    }
}

impl ThInstance {
    pub fn create_sdl3_surface(
        self: &Arc<ThInstance>,
        window: *mut SDL_Window,
    ) -> Result<Arc<ThSdl3Surface>, ()> {
        let mut handle = ptr::null_mut();

        let ok = unsafe {
            sdl3_sys::vulkan::SDL_Vulkan_CreateSurface(
                window,
                self.handle.handle().as_raw() as _,
                ptr::null(),
                &mut handle as _,
            )
        };

        if !ok {
            return Err(());
        }

        Ok(Arc::new(ThSdl3Surface {
            handle: SurfaceKHR::from_raw(handle as _),
            instance: self.clone(),
        }))
    }
}

impl Drop for ThSdl3Surface {
    fn drop(&mut self) {
        unsafe {
            sdl3_sys::vulkan::SDL_Vulkan_DestroySurface(
                self.instance.handle.handle().as_raw() as _,
                self.handle.as_raw() as _,
                ptr::null(),
            )
        }
    }
}
