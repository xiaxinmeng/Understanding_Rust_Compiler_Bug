rust
#![feature(asm)]

pub unsafe fn foo(a: &[u8; 32], b: &mut [u8; 32]) {
    asm!(
        "vmovups {ymm}, ymmword ptr [{a}]",
        "vmovups ymmword ptr [{b}], {ymm}",
        a = in(reg) a.as_ptr(),
        b = in(reg) b.as_mut_ptr(),
        ymm = out(ymm_reg) _,
    );
}
