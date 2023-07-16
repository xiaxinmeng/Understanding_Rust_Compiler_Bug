 rust
pub struct Unique<T: ?Sized> {
    pub ptr: *mut T,
}
