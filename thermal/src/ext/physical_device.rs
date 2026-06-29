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
    fn sort_by_type(self) -> Vec<ThPhysicalDevice>;

    fn find_with_queue_family<P: FnMut(&ThPhysicalDevice, u32, QueueFlags) -> bool>(
        &mut self,
        predicate: P,
    ) -> Option<(ThPhysicalDevice, u32)>;

    fn find_with_graphics_queue_family(&mut self) -> Option<(ThPhysicalDevice, u32)>;
}

impl<T: Iterator<Item = ThPhysicalDevice>> ThPhysicalDeviceIteratorExt for T {
    fn sort_by_type(self) -> Vec<ThPhysicalDevice> {
        let mut devices = self.collect::<Vec<_>>();

        devices.sort_by_cached_key(|device| match device.properties().device_type {
            PhysicalDeviceType::DISCRETE_GPU => 4,
            PhysicalDeviceType::INTEGRATED_GPU => 3,
            PhysicalDeviceType::VIRTUAL_GPU => 2,
            PhysicalDeviceType::CPU => 1,
            PhysicalDeviceType::OTHER => 0,
            _ => panic!("unknown device type"),
        });

        devices
    }

    fn find_with_queue_family<P: FnMut(&ThPhysicalDevice, u32, QueueFlags) -> bool>(
        &mut self,
        mut predicate: P,
    ) -> Option<(ThPhysicalDevice, u32)> {
        self.find_map(|device| {
            device
                .find_queue_family(&mut predicate)
                .map(|family| (device, family))
        })
    }

    fn find_with_graphics_queue_family(&mut self) -> Option<(ThPhysicalDevice, u32)> {
        self.find_with_queue_family(|_, _, flags| flags.contains(QueueFlags::GRAPHICS))
    }
}
