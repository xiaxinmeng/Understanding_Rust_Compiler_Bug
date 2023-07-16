rust
impl<T> Box<MaybeUninit<T>> {
    pub fn new_uninit() -> Self {…}
    pub unsafe fn into_initialized(b: Self) -> Box<T> { transmute(b) }
}

impl<T> Box<[MaybeUninit<T>]> {
    pub fn new_uninit_slice(len: usize) -> Self {…}
    pub unsafe fn into_initialized_slice(b: Self) -> Box<[T]> { transmute(b) }
}
