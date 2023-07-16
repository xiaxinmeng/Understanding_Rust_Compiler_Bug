rust
pub trait ConstSizeOf: Sized {
    const SIZE: usize = ::std::mem::size_of::<Self>();
}

impl<T> ConstSizeOf for T { }
