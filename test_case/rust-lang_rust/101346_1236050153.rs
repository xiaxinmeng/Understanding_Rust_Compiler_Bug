rust
pub fn foo() -> u32 {
    let t1: u32;
    let t2: u32;
    unsafe {
        asm!(
            "mov {0}, 1",
            "mov eax, 42",
            lateout(reg) t1,
            lateout("eax") t2, // `t2` can be replaced by `_`
            options(nostack),
        );
    }
    t1
}
