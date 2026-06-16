use std::sync::Arc;

use ash::vk::{BufferUsageFlags, MemoryPropertyFlags};

use crate::thvk::{
    device::ThDevice, device_buffer::ThDeviceBuffer, physical_device::ThPhysicalDevice,
};

pub struct ThVertexBuffer<T> {
    pub physical_device: ThPhysicalDevice,

    pub device: Arc<ThDevice>,

    vertices: Vec<T>,

    buffers: Vec<ThDeviceBuffer>,

    pub last_buffer: ThDeviceBuffer,

    last_capacity: u64,
}

impl<T: Copy> ThVertexBuffer<T> {
    pub fn new(physical_device: ThPhysicalDevice, device: Arc<ThDevice>, capacity: u64) -> Self {
        let buffer = Self::create_buffer(&physical_device, &device, capacity);

        Self {
            physical_device,
            device,
            vertices: vec![],
            buffers: vec![],
            last_buffer: buffer,
            last_capacity: capacity,
        }
    }

    pub fn add(&mut self, slice: &[T]) -> u32 {
        let size = (self.vertices.len() + slice.len()) as u64;

        if size <= self.last_capacity {
            let index = self.vertices.len();

            self.vertices.copy_from_slice(slice);

            return index as u32;
        }

        self.flush();

        let capacity = size.next_power_of_two();

        self.grow(capacity);

        self.vertices.copy_from_slice(slice);

        0
    }

    pub fn flush(&mut self) {
        self.last_buffer.memory().copy_from(&self.vertices).unwrap();

        self.vertices.clear();
    }

    pub fn clear(&mut self) {
        self.vertices.clear();

        self.buffers.clear();
    }

    fn grow(&mut self, capacity: u64) {
        let buffer = Self::create_buffer(&self.physical_device, &self.device, capacity);

        let last_buffer = std::mem::replace(&mut self.last_buffer, buffer);

        self.buffers.push(last_buffer);

        self.last_capacity = capacity;
    }

    fn create_buffer(
        physical_device: &ThPhysicalDevice,
        device: &Arc<ThDevice>,
        capacity: u64,
    ) -> ThDeviceBuffer {
        device
            .allocate_buffer(
                physical_device,
                capacity * size_of::<T>() as u64,
                BufferUsageFlags::VERTEX_BUFFER,
                MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            )
            .unwrap()
    }
}
