use std::{ops::Deref, sync::Arc};

use ash::vk::Handle;

use crate::thvk::device::ThDevice;

pub use thermal_derive::ThDeviceHandle;

pub trait ThHandle<T: Handle> {
    fn handle(&self) -> T;
}

pub trait ThDeviceHandle<T: Handle>: ThHandle<T> {
    fn device(&self) -> &Arc<ThDevice>;
}

pub trait ThSourceHandle<T: Handle>: ThDeviceHandle<T> + Clone {}

impl<TType: Handle, THandle: ThHandle<TType>> ThHandle<TType> for Arc<THandle> {
    fn handle(&self) -> TType {
        self.deref().handle()
    }
}

impl<TType: Handle, THandle: ThDeviceHandle<TType>> ThDeviceHandle<TType> for Arc<THandle> {
    fn device(&self) -> &Arc<ThDevice> {
        self.deref().device()
    }
}

impl<TType: Handle, THandle: ThDeviceHandle<TType> + Clone> ThSourceHandle<TType> for THandle {}
