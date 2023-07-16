rust
pub const unsafe fn from_bytes_unchecked<T>(bytes: &[u8]) -> &[T] {
    let (data, mut metadata): (*const u8, usize) = core::mem::transmute(bytes);
    metadata /= core::mem::size_of::<T>();
    core::mem::transmute((data, metadata))
}
