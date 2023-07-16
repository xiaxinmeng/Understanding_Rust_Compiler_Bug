
std::arch::asm!("
    mov r8, fs:{}@TPOFF
    mov rax, [r8]
", sym BAR)
