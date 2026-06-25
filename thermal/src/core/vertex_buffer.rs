use std::sync::Arc;

use ash::vk::{Buffer, BufferUsageFlags, MemoryPropertyFlags};

use crate::thvk::{
    buffer::ThBuffer,
    device::ThDevice,
    device_memory::ThDeviceMemory,
    handle::ThHandle,
    memory_mapping::{MemoryMappableExt, ThMemoryMapping},
};

pub struct VertexBuffer<T: Clone> {
    device: Arc<ThDevice>,

    vertices: Vec<T>,

    old_buffers: Vec<ThBuffer<ThDeviceMemory>>,

    last_buffer: ThMemoryMapping<ThBuffer<ThDeviceMemory>>,

    last_capacity: u32,

    index: u32,
}

impl<T: Clone> VertexBuffer<T> {
    pub fn new(device: Arc<ThDevice>, capacity: u32) -> Self {
        let buffer = Self::create_buffer(&device, capacity);

        Self {
            device,
            vertices: vec![],
            old_buffers: vec![],
            last_buffer: buffer,
            last_capacity: capacity,
            index: 0,
        }
    }

    pub fn device(&self) -> &Arc<ThDevice> {
        &self.device
    }

    pub fn push(&mut self, slice: &[T]) {
        self.vertices.extend_from_slice(slice);
    }

    pub fn draw_flush(&mut self) -> (Buffer, u32, u32) {
        let mut index = self.index;

        if self.vertices.len() > self.last_capacity as usize {
            let capacity = self.vertices.len() as u32;

            self.last_buffer.copy_from(&self.vertices[..index as usize]);

            self.vertices.drain(..index as usize);

            self.grow(capacity.next_power_of_two());

            index = 0;
        }

        self.index = self.vertices.len() as u32;

        (
            self.last_buffer.memory().handle(),
            index,
            self.vertices.len() as u32 - index,
        )
    }

    pub fn flush(&mut self) {
        self.last_buffer.copy_from(&self.vertices);

        self.vertices.clear();

        self.index = 0;
    }

    pub fn clear(&mut self) {
        self.vertices.clear();

        self.old_buffers.clear();

        self.index = 0;
    }

    fn grow(&mut self, capacity: u32) {
        let buffer = Self::create_buffer(&self.device, capacity);

        let old_buffer = std::mem::replace(&mut self.last_buffer, buffer);

        self.old_buffers.push(old_buffer.unmap());

        self.last_capacity = capacity;
    }

    fn create_buffer(
        device: &Arc<ThDevice>,
        capacity: u32,
    ) -> ThMemoryMapping<ThBuffer<ThDeviceMemory>> {
        let buffer = device
            .allocate_buffer(
                capacity as u64 * size_of::<T>() as u64,
                BufferUsageFlags::VERTEX_BUFFER,
                MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            )
            .unwrap();

        buffer.map_memory().unwrap()
    }
}
