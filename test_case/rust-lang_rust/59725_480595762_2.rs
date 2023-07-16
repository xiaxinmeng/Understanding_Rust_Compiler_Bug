rust
pub struct RawWakerVTable<T: ?Sized = ()> {
    clone: unsafe fn(*const T) -> RawWaker,
    wake: unsafe fn(*const T),
    wake_by_ref: unsafe fn(*const T),
    drop: unsafe fn(*const T),
}
