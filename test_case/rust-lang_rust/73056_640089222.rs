rust
#![feature(asm)]

pub fn black_box<T>(mut dummy: T) -> T {
    unsafe {
        asm!("", in(reg) &mut dummy, options(nostack, preserves_flags));
        dummy
    }
}
