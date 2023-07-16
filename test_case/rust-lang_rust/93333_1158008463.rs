
asm!(
    "movq rax, [rip + {}@GOTPCREL];
    # use the address stored in rax...",
    reg("rax") _,
    sym FOO,
);
