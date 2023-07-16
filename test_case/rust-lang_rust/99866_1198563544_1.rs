Rust
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
pub use gfx_hal as hal;

#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

#[cfg(not(any(feature = "metal", feature = "vulkan")))]
use gfx_backend_empty as back;

pub type B = back::Backend;

pub struct Exacterator<I: Iterator<Item = T>, T> {
    iter: I,
    size: usize,
}

impl<I: Iterator<Item = T>, T> Exacterator<I, T> {
    pub fn new(iter: I, size: usize) -> Self {
        Self { iter, size }
    }
}

impl<I: Iterator<Item = T>, T> Iterator for Exacterator<I, T> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<I: Iterator<Item = T>, T> ExactSizeIterator for Exacterator<I, T> {}

pub trait Exact<T>: Iterator<Item = T> + Sized {
    fn exact(self, size: usize) -> Exacterator<Self, T>;
}

impl<I: Iterator<Item = T>, T> Exact<T> for I {
    fn exact(self, size: usize) -> Exacterator<Self, T> {
        Exacterator::new(self, size)
    }
}

#[derive(Debug)]
pub struct HalSetLayouts {
    vertex_layout: <B as hal::Backend>::DescriptorSetLayout,
    fragment_layout: <B as hal::Backend>::DescriptorSetLayout,
    fragment_sampler_layout: <B as hal::Backend>::DescriptorSetLayout,
}

impl HalSetLayouts {
    pub fn iter<DSL>(&self) -> impl ExactSizeIterator<Item = &DSL> + '_
    where
        // rust 1.37 fails to typecheck `iter` without this. This is fixed in nightly.
        B: hal::Backend<DescriptorSetLayout = DSL>,
        DSL: std::fmt::Debug + Send + Sync + 'static,
    {
        std::iter::once(&self.vertex_layout)
            .chain(std::iter::once(&self.fragment_layout))
            .chain(std::iter::once(&self.fragment_sampler_layout))
            .exact(3)
    }
}
