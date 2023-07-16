plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
    |
541 |         U::from(self)
    |         ^^^^^^^^^^^^^


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
     |
     |
1917 |             Err(e) => Err(From::from(e)),

For more information about this error, try `rustc --explain E0015`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:05:06
