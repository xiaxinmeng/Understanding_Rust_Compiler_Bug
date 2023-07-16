rust
pub fn zeroize(x: &mut [u8]) {
    x.fill(0);
    unsafe {
        core::arch::asm!(
            "/* {ptr} */",
            ptr = in(reg) x.as_ptr(),
            options(nostack, readonly, preserves_flags),
        );
    }
}
