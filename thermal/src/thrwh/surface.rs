use std::sync::Arc;

use ash::{VkResult, vk::SurfaceKHR};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use thermal_derive::ThHandle;

use crate::thrwh::surface_factory::ThRwhSurfaceFactory;

#[derive(ThHandle)]
pub struct ThRwhSurface<TDisplay: HasDisplayHandle, TWindow: HasWindowHandle> {
    handle: SurfaceKHR,

    factory: Arc<ThRwhSurfaceFactory<TDisplay>>,

    window_handle: TWindow,
}

impl<TDisplay: HasDisplayHandle> ThRwhSurfaceFactory<TDisplay> {
    pub fn create_rwh_surface<TWindow: HasWindowHandle>(
        self: &Arc<ThRwhSurfaceFactory<TDisplay>>,
        window_handle: TWindow,
    ) -> VkResult<ThRwhSurface<TDisplay, TWindow>> {
        let handle = unsafe {
            self.handle()
                .create_surface(window_handle.window_handle().unwrap().as_raw(), None)
        }?;

        Ok(ThRwhSurface {
            handle,
            factory: self.clone(),
            window_handle,
        })
    }
}

impl<TDisplay: HasDisplayHandle, TWindow: HasWindowHandle> ThRwhSurface<TDisplay, TWindow> {
    pub fn factory(&self) -> &Arc<ThRwhSurfaceFactory<TDisplay>> {
        &self.factory
    }

    pub fn window_handle(&self) -> &TWindow {
        &self.window_handle
    }
}

impl<TDisplay: HasDisplayHandle, TWindow: HasWindowHandle> Drop
    for ThRwhSurface<TDisplay, TWindow>
{
    fn drop(&mut self) {
        unsafe {
            self.factory
                .instance()
                .surface_instance
                .destroy_surface(self.handle, None)
        }
    }
}
