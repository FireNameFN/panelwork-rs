use std::sync::Arc;

use ash::{
    VkResult,
    vk::{CommandBuffer, Fence, PipelineStageFlags, Queue, Semaphore, SubmitInfo},
};

use crate::thvk::device::ThDevice;

#[derive(Clone)]
pub struct ThQueue {
    pub handle: Queue,

    pub device: Arc<ThDevice>,

    pub family: u32,
}

impl ThDevice {
    pub fn get_queue(self: &Arc<ThDevice>, family: u32, index: u32) -> ThQueue {
        let handle = unsafe { self.handle.get_device_queue(family, index) };

        ThQueue {
            handle,
            device: self.clone(),
            family,
        }
    }
}

impl ThQueue {
    pub fn submit(
        &self,
        fence: Fence,
        wait_semaphores: &[Semaphore],
        wait_stages: &[PipelineStageFlags],
        command_buffers: &[CommandBuffer],
        signal_semaphores: &[Semaphore],
    ) -> VkResult<()> {
        let submit_info = SubmitInfo {
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: command_buffers.len() as u32,
            p_command_buffers: command_buffers.as_ptr(),
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            ..Default::default()
        };

        unsafe {
            self.device
                .handle
                .queue_submit(self.handle, &[submit_info], fence)
        }
    }

    pub fn wait_idle(&self) -> VkResult<()> {
        unsafe { self.device.handle.queue_wait_idle(self.handle) }
    }
}
