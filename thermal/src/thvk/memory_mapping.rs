use std::ffi::c_void;

use ash::{
    VkResult,
    vk::{self, DeviceMemory, MemoryMapFlags},
};

use crate::thvk::handle::{ThDeviceHandle, ThHandle};

pub trait MemoryMappable: Sized {
    type Memory: ThDeviceHandle<DeviceMemory>;

    fn memory(&self) -> &Self::Memory;
}

pub trait MemoryMappableExt: MemoryMappable {
    fn map_memory(self) -> VkResult<ThMemoryMapping<Self>>;
}

impl<T: MemoryMappable> MemoryMappableExt for T {
    fn map_memory(self) -> VkResult<ThMemoryMapping<Self>> {
        let memory = self.memory();

        let mapping = unsafe {
            memory.device().handle.map_memory(
                memory.handle(),
                0,
                vk::WHOLE_SIZE,
                MemoryMapFlags::empty(),
            )
        }?;

        Ok(ThMemoryMapping {
            mapping,
            memory: self,
        })
    }
}

pub struct ThMemoryMapping<T: MemoryMappable> {
    mapping: *mut c_void,

    memory: T,
}

impl<T: MemoryMappable> ThMemoryMapping<T> {
    pub fn mapping(&self) -> *mut c_void {
        self.mapping
    }

    pub fn memory(&self) -> &T {
        &self.memory
    }

    pub fn unmap(self) -> T {
        let memory = self.memory.memory();

        unsafe { memory.device().handle.unmap_memory(memory.handle()) };

        self.memory
    }

    pub fn copy_from(&self, slice: &[impl Clone]) {
        unsafe {
            self.mapping
                .copy_from_nonoverlapping(slice.as_ptr().cast(), std::mem::size_of_val(slice))
        };
    }
}
