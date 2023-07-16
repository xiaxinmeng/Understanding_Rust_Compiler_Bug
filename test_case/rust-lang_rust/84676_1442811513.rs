rust
unsafe {
    core::arch::asm!(
        "
        .code16
        ljmp 0, offset 1f
        1:
        int3
        .code64
        ",
        options(noreturn)
    )
}
