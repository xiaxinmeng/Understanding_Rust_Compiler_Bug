rust
struct MaybeInitialized<T> {
    _align: [T; 0],
    data: [u8; size_of::<T>()],
}
