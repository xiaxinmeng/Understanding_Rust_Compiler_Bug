rust
extern {
    fn memcpy(dest: *mut Whatever, src: *const Whatever, n: size_t);
}

unsafe impl GlobalAlloc for BetterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut Whatever {
        /* ... */ as *mut Whatever
    }
}
