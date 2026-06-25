pub mod rect;

pub trait Mesh<T: Clone, const N: usize> {
    fn vertices(&self) -> [T; N];
}
