rust
pub fn new_uninit_slice(len: usize) -> Box<[mem::MaybeUninit<T>]> {
    unsafe { RawVec::with_capacity(len).into_box(len) }
}
