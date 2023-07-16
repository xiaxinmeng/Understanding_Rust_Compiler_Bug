rs
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait True {}
pub struct Assert<const COND: bool>;
impl True for Assert<true> {}

pub struct InitialPageAllocator;

impl InitialPageAllocator {
    const PAGE_SIZE: usize = 4 << 10;

    fn allocate_aligned<const SIZE: usize>(&mut self)
    where
        Assert<{ SIZE % Self::PAGE_SIZE == 0 }>: True,
    {
        self.free::<{ 64 * Self::PAGE_SIZE }>();
    }

    fn free<const SIZE: usize>(&mut self)
    where
        Assert<{ SIZE % Self::PAGE_SIZE == 0 }>: True,
    {
    }
}
