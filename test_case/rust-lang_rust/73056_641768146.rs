rust
// Works
#[allow(unused_asm_arguments)]
{
    asm!("", in(reg) 0, in(reg) 1);
}

// Doesn't work
#[allow(unused_asm_arguments)]
asm!("", in(reg) 0, in(reg) 1);
