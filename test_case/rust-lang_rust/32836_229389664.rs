 rust
union SlotInner<V> {
    next_empty: usize, /* index of next empty slot */
    value: V,
}

struct Slot<V> {
    inner: SlotInner<V>,
    version: u64 /* even version -> is_empty */
}
