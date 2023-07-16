rust
unsafe fn g<T: ?Sized>(ptr: *mut T, new: *mut u8) {
    const S: usize = ::std::mem::size_of::<*mut T>();
    let mut parts: [u8; S];
}
