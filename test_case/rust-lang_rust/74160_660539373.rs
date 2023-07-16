rust
unsafe fn align_of_val_raw<T: ?Sized>(val: *const T) -> usize {
    if T is Sized {
        mem::align_of::<T>()
    } else {
        mem::align_of_val::<T>(&*val)
    }
}
