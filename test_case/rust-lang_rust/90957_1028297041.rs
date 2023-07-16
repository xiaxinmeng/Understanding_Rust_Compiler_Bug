rust
const THREE: usize = 3;

#[naked]
/// Adds three to a number and returns the result.
pub extern "sysv64" fn add_n(number: usize) -> usize {
    // SAFETY: the validity of these registers is guaranteed according to the "sysv64" ABI
    unsafe {
        std::arch::asm!(
            "add rdi, {}",
            "mov rax, rdi",
            "ret",
            const THREE,
            options(noreturn)
        );
    }
}
