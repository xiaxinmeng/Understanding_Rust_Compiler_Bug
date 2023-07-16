rust
use core::ptr;
use core::mem;

// Assumes it's safe to bypass the ZST's constructor(s)
unsafe fn zst_box<T>() -> Box<T> {
    assert_eq!(mem::size_of::<T>(), 0);
    // `NonNull::dangling` ensures that the pointer is well-aligned
    // and does not come from an allocator.
    let ptr = ptr::NonNull::dangling().as_ptr();
    Box::from_raw(ptr)
}
