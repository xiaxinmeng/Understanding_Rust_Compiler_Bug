rust
asm!("
    push {{r7, r11}}
    mov r7, {dest}
    mov r11, {sysnum}
    svc #0
    pop {{r7, r11}}
    ",
    dest = in(reg) dest.as_mut_ptr(),
    sysnum = const Sysnum::BorrowRead as u32,
);
