rust
let ptr = usize::MAX as *const u8;
unsafe {
    assert_eq!(ptr.add(1), core::ptr::null());
}
