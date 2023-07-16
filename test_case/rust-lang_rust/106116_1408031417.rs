rust
pub const fn null<T>() -> *const T
where
    T: Pointee + ?Sized,
    <T as Pointee>::Metadata: MetadataForNull,
