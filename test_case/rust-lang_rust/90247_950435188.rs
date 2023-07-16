plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: no method named `trunc` found for type `f64` in the current scope
    |
    |
753 |         let full_secs = secs.trunc();
    |                              ^^^^^ method not found in `f64`

error[E0599]: no method named `trunc` found for type `f32` in the current scope
    |
    |
815 |         let full_secs = secs.trunc();
    |                              ^^^^^ method not found in `f32`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:08
