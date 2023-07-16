plain
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
     |
1947 |     unsafe { copy(src, dst, count) }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0015`.
