rust
#[repr(C)]
pub struct SafeVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize
}
