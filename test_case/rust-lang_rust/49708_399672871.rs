rust
fn size_of_val<T: ?Sized>(val: &T) -> usize {
    if is_extern_type::<T>() {
        panic!(...)
    } else {
        intrinsics::size_of_val(val)
    }
}
