use std::sync::Arc;

use ash::vk::CommandPool;

use crate::{
    core::vertex_buffer::VertexBuffer,
    mesh::Mesh,
    thvk::{command_buffer::ThCommandBuffer, device::ThDevice, handle::ThDeviceHandle},
};

pub struct DrawHandle<TVertex: Clone, TInstance: Clone> {
    vertex_buffer: VertexBuffer<TVertex>,

    instance_buffer: VertexBuffer<TInstance>,
}

impl<TVertex: Clone, TInstance: Clone> DrawHandle<TVertex, TInstance> {
    pub fn new(device: Arc<ThDevice>) -> Self {
        Self {
            vertex_buffer: VertexBuffer::new(device.clone(), 16),
            instance_buffer: VertexBuffer::new(device, 4),
        }
    }

    pub fn add<const N: usize>(&mut self, mesh: &impl Mesh<TVertex, N>) {
        self.vertex_buffer.push(&mesh.vertices());
    }

    pub fn set_instance(&mut self, instance: &[TInstance]) {
        self.instance_buffer.push(instance);
    }

    pub fn draw<T: ThDeviceHandle<CommandPool>>(&mut self, command_buffer: &ThCommandBuffer<T>) {
        let (vertex_buffer, vertex_index, vertex_count) = self.vertex_buffer.draw_flush();
        let (instance_buffer, instance_index, instance_count) = self.instance_buffer.draw_flush();

        command_buffer.cmd_bind_vertex_buffers(0, &[vertex_buffer, instance_buffer], &[0, 0]);

        command_buffer.cmd_draw(vertex_count, instance_count, vertex_index, instance_index);
    }

    pub fn flush(&mut self) {
        self.vertex_buffer.flush();
        self.instance_buffer.flush();
    }
}
