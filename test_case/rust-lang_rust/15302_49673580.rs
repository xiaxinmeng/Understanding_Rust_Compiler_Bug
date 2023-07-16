
pub fn with_capacity(capacity: uint) -> Vec<T> {
    [...] {
        let size = capacity.checked_mul(&mem::size_of::<T>())
                           .expect("capacity overflow");
        let ptr = unsafe { allocate(size, mem::min_align_of::<T>()) };
        Vec { len: 0, cap: capacity, ptr: ptr as *mut T }
    }
}
