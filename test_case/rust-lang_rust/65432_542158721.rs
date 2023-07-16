rust
pub union MaybeUninit<T> {
    uninit: (),
    value: ManuallyDrop<T>,
}
