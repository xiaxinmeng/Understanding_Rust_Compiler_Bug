rust
#[naked]
pub extern "C" fn i_am_naked() {
    unsafe {
        #[cfg(target_arch = "arm")]
        asm!("bx lr", options(noreturn));

        #[cfg(target_arch = "riscv")]
        asm!("ret", options(noreturn));

        #[cfg(target_arch = "x86_64")]
        asm!("ret", options(noreturn));
    }
}
