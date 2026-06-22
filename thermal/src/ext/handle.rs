use std::sync::Arc;

use ash::vk::Handle;

use crate::thvk::handle::ThDeviceHandle;

pub trait ThHandleDeviceExt<T: Handle>: ThDeviceHandle<T> {
    fn arc(self) -> Arc<Self>;
}

impl<TType: Handle, THandle: ThDeviceHandle<TType>> ThHandleDeviceExt<TType> for THandle {
    fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
