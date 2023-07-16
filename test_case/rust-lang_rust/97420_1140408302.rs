plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0606]: casting `&*const T` as `*const ()` is invalid
     |
     |
2237 |         pointer_fmt_inner((self as *const ()).addr(), f)

For more information about this error, try `rustc --explain E0606`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
