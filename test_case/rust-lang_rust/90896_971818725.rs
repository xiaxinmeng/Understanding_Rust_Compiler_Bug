rust
impl<T> MaybeUninit<T> {
    pub const fn as_ptr(&self) -> *const T;
    pub const unsafe fn assume_init(self) -> T;
    pub const unsafe fn assume_init_ref(&self) -> &T;
}
