rust
#[no_mangle]
pub fn foo(accum: u64, output: &mut [u8;6]) {
    unsafe {
        let bytes = *(&accum.to_be() as *const _ as *const [u8; 8]);
        ::std::ptr::copy_nonoverlapping(bytes.as_ptr(), output.as_mut_ptr(), 6);
    }
}
