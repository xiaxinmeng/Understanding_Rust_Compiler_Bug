rust
fn align_offset_aligned<T>(ptr: &[T; 0], align: usize) -> usize
where
    T: Sized,
{
    if !align.is_power_of_two() {
        panic!("align_offset: align is not a power-of-two");
    }

    let byte_align_offset = ptr.as_ptr().cast::<u8>().align_offset(align);
    byte_align_offset / core::mem::align_of::<T>()
}
