plain
   Compiling libc v0.2.98
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: use of deprecated macro `llvm_asm`: will be removed from the compiler, use asm! instead
    |
    |
170 |         llvm_asm!("" : : "r"(&mut dummy) : "memory" : "volatile");
    |
    |
    = note: `-D deprecated` implied by `-D warnings`
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:07
