rust
impl Box<T: ?Sized, A> {
    pub unsafe fn cast_unchecked<U>(self) -> Box<U, A> {
        debug_assert_eq!(Layout::for_value(&*self), Layout::new::<U>());
        unsafe {
            let (raw, alloc) = Box::into_raw_with_allocator(self);
            Box::from_raw_in(raw.cast(), alloc)
        }
    }
}
