rust
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        asm!("", options(nostack, preserves_flags));
        dummy
    }
}
