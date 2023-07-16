rust
    asm!(
        "lock",
        "xchg [{0}], {1:e}",
        in(reg) addr,
        inout(reg) newval => result,
    );
