rust
pub fn uwu() {
    unsafe {
        core::arch::asm!("", in("ah") 1_u8, in("al") 0_u8);
    }
}
