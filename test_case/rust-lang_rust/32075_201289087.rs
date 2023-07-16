 rust
// In the alloc::heap module
pub unsafe fn usable_size(ptr: *mut u8, size: usize, align: usize) -> usize { ... }
