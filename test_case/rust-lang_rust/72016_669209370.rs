
struct Registers {
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    r8: usize,
    r9: usize,
    r10: usize,
    r11: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
}

struct Context {
    // ... other fields
    registers: Registers
    // ... other fields
}

unsafe extern "C" fn handler(
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    r8: usize,
    r9: usize,
    ret: usize,
    ctx: &mut Context
) -> usize {
    let r10: usize;
    let r11: usize;
    let r12: usize;
    let r13: usize;
    let r14: usize;
    let r15: usize;

    asm!(
        "",
        lateout("r10") r10,
        lateout("r11") r11,
        lateout("r12") r12,
        lateout("r13") r13,
        lateout("r14") r14,
        lateout("r15") r15,
        options(pure, nomem, nostack)
    );

    ctx.registers.rdi = rdi;
    ctx.registers.rsi = rsi;
    ctx.registers.rdx = rdx;
    ctx.registers.rcx = rcx;
    ctx.registers.r8 = r8;
    ctx.registers.r9 = r9;
    ctx.registers.r10 = r10;
    ctx.registers.r11 = r11;
    ctx.registers.r12 = r12;
    ctx.registers.r13 = r13;
    ctx.registers.r14 = r14;
    ctx.registers.r15 = r15;

    ret
}
