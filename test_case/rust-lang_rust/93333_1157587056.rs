rust
#[naked]
unsafe extern "C" fn foo() {
    asm!(
        "jmp rax",
        in("rax") TLS_VAR,
    )
}
