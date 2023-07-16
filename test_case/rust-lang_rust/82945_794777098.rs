rust
#[inline(never)]
fn fmt_u32() {
    static DEC_DIGITS_LUT: &[u8; 6] = b"424242";
    let mut buf = [0; 5];
    let buf_ptr = buf.as_ptr() as *mut u8;
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();

    // Crashes if n is odd
    let n = 1;

    unsafe {
        core::ptr::copy_nonoverlapping(lut_ptr, buf_ptr.offset(n), 2);
    }

    let buf_slice = unsafe {
        core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(buf_ptr.offset(n), 2))
    };
    // Force usage of `buf_slice`
    write!(DummyWrite, "Test: {}", buf_slice).unwrap();
}
