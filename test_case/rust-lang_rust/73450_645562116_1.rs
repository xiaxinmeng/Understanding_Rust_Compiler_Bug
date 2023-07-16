rust
asm!("
    mov {save7}, r7
    move {save11}, r11
    mov r7, {dest}
    mov r11, {sysnum}
    svc #0
    mov r7, {save7}
    move r11, {save11}
    ",
    dest = in(reg) dest.as_mut_ptr(),
    sysnum = const Sysnum::BorrowRead as u32,
    save7 = out(reg) _,
    save11 = out(reg) _,
    options(nostack),
);
