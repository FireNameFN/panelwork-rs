use std::sync::Arc;

use ash::{
    VkResult,
    vk::{
        MemoryPropertyFlags, PhysicalDevice, PhysicalDeviceMemoryProperties,
        PhysicalDeviceProperties, PresentModeKHR, QueueFamilyProperties, SurfaceCapabilitiesKHR,
        SurfaceKHR,
    },
};

use crate::thvk::instance::ThInstance;

#[derive(Clone)]
pub struct ThPhysicalDevice {
    pub handle: PhysicalDevice,

    pub instance: Arc<ThInstance>,
}

impl ThPhysicalDevice {
    pub fn properties(&self) -> PhysicalDeviceProperties {
        unsafe {
            self.instance
                .handle
                .get_physical_device_properties(self.handle)
        }
    }

    pub fn memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance
                .handle
                .get_physical_device_memory_properties(self.handle)
        }
    }

    pub fn queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        unsafe {
            self.instance
                .handle
                .get_physical_device_queue_family_properties(self.handle)
        }
    }

    pub fn surface_capabilities(&self, surface: SurfaceKHR) -> VkResult<SurfaceCapabilitiesKHR> {
        unsafe {
            self.instance
                .surface_instance
                .get_physical_device_surface_capabilities(self.handle, surface)
        }
    }

    pub fn surface_present_modes(&self, surface: SurfaceKHR) -> VkResult<Vec<PresentModeKHR>> {
        unsafe {
            self.instance
                .surface_instance
                .get_physical_device_surface_present_modes(self.handle, surface)
        }
    }

    pub fn find_memory_type(&self, mask: u32, properties: MemoryPropertyFlags) -> Option<u32> {
        let memory_properties = self.memory_properties();

        memory_properties
            .memory_types_as_slice()
            .iter()
            .enumerate()
            .filter(|(i, _)| (1 << i & mask) != 0)
            .filter(|(_, memory_type)| memory_type.property_flags.contains(properties))
            .next()
            .map(|(i, _)| i as u32)
    }
}
