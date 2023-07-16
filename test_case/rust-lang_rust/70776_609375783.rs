rust

fn from(slice: &[T]) -> Box<[T]> {
    let len = slice.len();
    let buf = RawVec::with_capacity(len);
    unsafe {
        ptr::copy_nonoverlapping(slice.as_ptr(), buf.ptr(), len);
        buf.into_box(len).assume_init()
    }
}
