
pub fn black_box<T>(mut dummy: T) -> T {
    unsafe {
        asm!("/* make it look like {} is accessed by an external agent */",
             in(reg) &mut dummy, options(nostack, preserves_flags));
        dummy
    }
}
