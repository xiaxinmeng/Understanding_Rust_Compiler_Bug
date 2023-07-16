rust
struct SmallArray<T: Sized>([u8; core::mem::size_of::<T>()]);
