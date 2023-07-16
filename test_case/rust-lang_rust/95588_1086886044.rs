rust
std::arch::asm!(
    "/* {0} */",
    in(reg) ptr,
    options(nostack),
)
