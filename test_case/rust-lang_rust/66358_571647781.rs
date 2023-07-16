rust
impl<T: ?Sized> *mut UnsafeCell<T> {
    pub const fn raw_get(self) -> *mut T {
        UnsafeCell::raw_get(self)
    }
}

impl<T: ?Sized> *const UnsafeCell<T> {
    pub const fn raw_get(self) -> *mut T {
        UnsafeCell::raw_get(self)
    }
}
