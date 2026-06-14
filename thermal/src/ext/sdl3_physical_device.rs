use crate::{
    ext::physical_device::ThPhysicalDeviceIteratorExt, sdl3_util,
    thvk::physical_device::ThPhysicalDevice,
};

pub trait ThPhysicalDeviceSdl3IteratorExt: ThPhysicalDeviceIteratorExt {
    fn find_with_sdl_presentation_support(&mut self) -> Option<(ThPhysicalDevice, u32)>;
}

impl<T: ThPhysicalDeviceIteratorExt> ThPhysicalDeviceSdl3IteratorExt for T {
    fn find_with_sdl_presentation_support(&mut self) -> Option<(ThPhysicalDevice, u32)> {
        self.find_with_queue_family(|physical_device, family, _| {
            sdl3_util::sdl_presentation_support(
                physical_device.instance.handle.handle(),
                physical_device.handle,
                family,
            )
        })
    }
}
