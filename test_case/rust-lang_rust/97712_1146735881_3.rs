rust
#[repr(align(128))]
#[derive(Copy, Clone)]
pub struct A(u8);

pub unsafe fn typed_copy(src: *const A, dst: *mut A) {
    (*dst).0 = (*src).0;
}

pub unsafe fn untyped_copy(src: *const A, dst: *mut A) {
    std::ptr::copy_nonoverlapping(src, dst, 1);
}
