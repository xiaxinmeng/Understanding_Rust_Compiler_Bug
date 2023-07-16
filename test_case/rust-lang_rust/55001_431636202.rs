rust
pub struct DefaultAllocator;
pub struct Standard;
pub trait Rand {}

pub trait Distribution<T> {}
pub trait Allocator<N> {}

impl<T> Rand for T where Standard: Distribution<T> {}

impl<N> Distribution<Point<N>> for Standard 
where
DefaultAllocator: Allocator<N>,
Standard: Distribution<N> {
}


pub struct Point<N> 
where DefaultAllocator: Allocator<N>
{
    field: N
}
