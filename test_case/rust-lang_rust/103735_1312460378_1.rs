rust
pub fn test(ptr: *mut [u8]) -> *mut [u8] {
    let layout_size = 24;
    unsafe { addr_of_mut!((&mut *ptr)[..layout_size]) }
}
