use ash::vk::Handle;

pub trait ThHandle<T: Handle> {
    fn handle(&self) -> T;
}
