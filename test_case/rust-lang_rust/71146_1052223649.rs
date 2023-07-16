rust
/// # Safety
/// The caller must ensure that the start of `ptr`:
/// 1. does not point to memory that was previously allocated but is now deallocated;
/// 2. must be within the bounds of a single allocated object.
pub unsafe fn ptr_len<T>(ptr: NonNull<[T]>) -> usize {
   (&*(ptr.as_ptr() as *const [()])).len()
}
