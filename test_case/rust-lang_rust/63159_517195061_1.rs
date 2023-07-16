rust
fn copy_nonoverlapping<T>(src: *const T, dest: *mut T, len: usize) {
    let src = src as *const MaybeUninit<u8>;
    let dest = dest as *mut MaybeUninit<u8>;
    for i in 0..(len * size_of::<T>()) { *dest.add(i) = *src.add(i); }
}
