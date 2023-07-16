rust
impl<T: ?Sized> UnsafeCell<T> {
     pub const fn raw_get(this: *const Self) -> *mut T;
}
