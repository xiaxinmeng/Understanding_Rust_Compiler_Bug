rust
#[inline(never)]
fn _bzhi_u32() { asm!("bzhi    eax, edi, esi; ....") }
