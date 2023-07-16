rust
// this is currently not overwriting the second byte
pub unsafe fn write_uninit(x: *mut MaybeUninit<(u8, u16)>) {
    let zeroed = [0u8; 4];
    *x = *(&zeroed as *const _ as *const _);
}
// this does zero all four
pub unsafe fn ptr_copy_uninit(x: *mut MaybeUninit<(u8, u16)>) {
    let zeroed = [0u8; 4];
    ptr::copy::<MaybeUninit<(u8, u16)>>(&zeroed as *const _ as *const _, x, 1);
}
