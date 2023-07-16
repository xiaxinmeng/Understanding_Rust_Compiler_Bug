 rust
/// Allocated but not-necessarily-initialized memory.
struct Buffer {
    ptr: Unique<u8>,
    size: usize,
    align: usize,
}

impl Drop for Buffer {
    /* deallocate */
}

impl Buffer {
    fn new(size: usize, align: usize) -> Result<Self, ()> {
        /* allocate, but avoid undefined behavior by asserting or something
           (maybe skip calling allocate() on `size == 0`?) */
    }

    // Maybe skip those and keep the unsafe functions API?
    fn into_ptr(self) -> *mut u8 { /* forget self */ }
    unsafe fn from_raw_parts(ptr: *mut u8, size: usize, align: usize) -> Self { /* */ }

    fn as_ptr(&self) -> *const u8 { self.ptr }
    fn as_mut_ptr(&mut self) -> *mut u8 { self.ptr }
    fn size(&self) -> usize { self.size }  // Call this len?
    fn align(&self) -> usize { self.align }

    // The caller is responsible for not reading uninitialized memory
    unsafe fn as_slice(&self) -> &[u8] { /* ... */ }  
    unsafe fn as_mut_slice(&mut self) -> &mut [u8] { /* ... */ }

    fn reallocate(&mut self, new_size: usize) -> Result<(), ()> { /* ... */ }
    fn reallocate_in_place(&mut self, new_size: usize) -> Result<(), ()> { /* ... */ }
}
