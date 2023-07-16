rust
pub fn test(buf: &mut [u8], src: &[u8]) {
    let amt = cmp::min(buf.len(), src.len());
    // Copy 0 or 1 bytes.
    let amt = cmp::min(amt, 1);
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), buf.as_mut_ptr(), amt);
    }
}
