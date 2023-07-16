Rust
#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
struct HashmapEntry<V> {
    // 1 bit occupied and 63 bit hash
    state: u64,
    value: V,
}
