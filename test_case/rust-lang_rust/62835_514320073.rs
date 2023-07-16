
impl<T> MaybeUninit<T> {
    fn copy(&self) -> Self {
        unsafe { ptr::read(self) }
    }
}
