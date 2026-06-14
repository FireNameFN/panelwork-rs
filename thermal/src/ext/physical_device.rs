use ash::vk::{PhysicalDeviceType, QueueFlags};

use crate::thvk::physical_device::ThPhysicalDevice;

impl ThPhysicalDevice {
    pub fn find_queue_family(
        &self,
        mut predicate: impl FnMut(&ThPhysicalDevice, u32, QueueFlags) -> bool,
    ) -> Option<u32> {
        self.queue_family_properties()
            .iter()
            .enumerate()
            .find_map(|(i, props)| predicate(self, i as u32, props.queue_flags).then_some(i as u32))
    }
}

pub trait ThPhysicalDeviceIteratorExt: Iterator<Item = ThPhysicalDevice> {
    fn filter_discrete(self) -> impl ThPhysicalDeviceIteratorExt;

    fn find_with_queue_family<P: FnMut(&ThPhysicalDevice, u32, QueueFlags) -> bool>(
        self,
        predicate: P,
    ) -> Option<(ThPhysicalDevice, u32)>;
}

impl<T: Iterator<Item = ThPhysicalDevice>> ThPhysicalDeviceIteratorExt for T {
    fn filter_discrete(self) -> impl ThPhysicalDeviceIteratorExt {
        self.filter(|device| {
            let properties = device.properties();

            properties.device_type == PhysicalDeviceType::DISCRETE_GPU
        })
    }

    fn find_with_queue_family<P: FnMut(&ThPhysicalDevice, u32, QueueFlags) -> bool>(
        mut self,
        mut predicate: P,
    ) -> Option<(ThPhysicalDevice, u32)> {
        self.find_map(|device| {
            device
                .find_queue_family(&mut predicate)
                .map(|family| (device, family))
        })
    }
}
