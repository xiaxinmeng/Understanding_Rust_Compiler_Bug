rust
#![feature(asm)]
#[allow(unused_assignments, unused_variables)]
#[cfg(any(target_arch = "x86_64"))]
pub unsafe fn mul_mod(this: u64, b: u64, m: u64) -> u64 {
    let mut q: u64;
    let mut r: u64; // r = a b mod m

    asm!(
        "mul {b}; div {m};",
        b = in(reg) b, m = in(reg) m,
        inout("rax") this => q, out("rdx") r,
        options(nostack, nomem)
    );
    r
}
