plain
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: constant is never used: `ASCII_DIGIT_MASK`
    |
    |
335 |         const ASCII_DIGIT_MASK: u32 = 0b11_0000;
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `core`

