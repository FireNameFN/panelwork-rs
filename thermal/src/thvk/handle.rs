use std::sync::Arc;

use ash::vk::Handle;

use crate::thvk::device::ThDevice;

pub trait ThHandle<T: Handle> {
    fn handle(&self) -> T;
}

pub trait ThHandleSource<T: Handle>: Clone {
    fn handle(&self) -> T;

    fn device(&self) -> &Arc<ThDevice>;
}

impl<TType: Handle, TSource: ThHandleSource<TType>> ThHandle<TType> for TSource {
    fn handle(&self) -> TType {
        self.handle()
    }
}
