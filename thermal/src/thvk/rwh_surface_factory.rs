use std::sync::Arc;

use ash::VkResult;
use ash_window::SurfaceFactory;
use raw_window_handle::HasDisplayHandle;

use crate::thvk::instance::ThInstance;

pub struct ThRwhSurfaceFactory<T: HasDisplayHandle> {
    handle: SurfaceFactory,

    instance: Arc<ThInstance>,

    display_handle: T,
}

impl ThInstance {
    pub fn create_rwh_surface_factory<T: HasDisplayHandle>(
        self: &Arc<ThInstance>,
        display_handle: T,
    ) -> VkResult<Arc<ThRwhSurfaceFactory<T>>> {
        let handle = SurfaceFactory::new(
            &self.library.entry,
            &self.handle,
            display_handle.display_handle().unwrap().as_raw(),
        )?;

        Ok(Arc::new(ThRwhSurfaceFactory {
            handle,
            instance: self.clone(),
            display_handle,
        }))
    }
}

impl<T: HasDisplayHandle> ThRwhSurfaceFactory<T> {
    pub fn handle(&self) -> &SurfaceFactory {
        &self.handle
    }

    pub fn instance(&self) -> &Arc<ThInstance> {
        &self.instance
    }

    pub fn display_handle(&self) -> &T {
        &self.display_handle
    }
}
