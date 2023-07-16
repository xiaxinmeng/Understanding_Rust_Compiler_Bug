rust
fn null<T: ?Sized + Thin>() -> *const T {
    0usize as *const T
}
