rust
pub struct RawWakerVTable<T: ?Sized = ()> {
    clone: unsafe fn(NonNull<T>) -> RawWaker,
    wake: unsafe fn(NonNull<T>),
    wake_by_ref: unsafe fn(NonNull<T>),
    drop: unsafe fn(NonNull<T>),
}
