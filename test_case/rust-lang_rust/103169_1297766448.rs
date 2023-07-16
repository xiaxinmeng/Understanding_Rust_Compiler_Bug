plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: cannot call non-const fn `intrinsics::abort` in constant functions
   |
71 |         super::intrinsics::abort()
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot call non-const fn `PanicInfo::internal_constructor` in constant functions
   |
   |
74 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location::caller(), Some(source), true);
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot call non-const fn `panicking::panic_impl` in constant functions
   |
   |
77 |     unsafe { panic_impl(&pi) }
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
For more information about this error, try `rustc --explain E0015`.
error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:02:12
