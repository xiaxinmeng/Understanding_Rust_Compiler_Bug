plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0522]: definition of an unknown language item: `const_panic_source`
   --> library/core/src/panicking.rs:201:1
    |
201 | #[lang = "const_panic_source"]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `const_panic_source`
For more information about this error, try `rustc --explain E0522`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:07
