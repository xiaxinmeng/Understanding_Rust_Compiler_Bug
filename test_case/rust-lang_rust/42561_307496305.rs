rust
fn is_aligned<T>(ptr: *mut T) -> bool {
    use core::mem::align_of;
    (ptr as usize) % align_of::<T>() == 0
}
