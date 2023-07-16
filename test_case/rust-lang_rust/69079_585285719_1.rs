rust
unsafe fn dealloc_t<T: ?Sized>(a: &mut impl AllocRef, ptr: NonNull<T>) {
    a.dealloc(ptr.cast(), Layout::for_ptr(ptr.as_ptr()))
}
