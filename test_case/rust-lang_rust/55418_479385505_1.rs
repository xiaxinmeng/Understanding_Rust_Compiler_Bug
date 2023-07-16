
#[inline]
#[unstable(feature = "renamed_spin_loop", issue = "55002")]
pub fn spin_loop() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        asm!("pause" ::: "memory" : "volatile");
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("yield" ::: "memory" : "volatile");
    }
}
