rust
unsafe { core::arch::asm!("mcr p0, #0, r0, 0, 0, #0", in("r0") value, options(nomem, nostack) }
