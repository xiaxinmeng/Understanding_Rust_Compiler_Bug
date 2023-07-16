rust
asm!(
    "fclass.s {}, {}",
    out(reg) ans,
    in(freg) input,
    options(pure, nomem, nostack)
);
