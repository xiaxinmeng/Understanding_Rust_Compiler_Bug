
 + ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
error: environment variable `CARGO_PKG_NAME` not defined
   --> ../library/core/src/../../std/src/primitive_docs.rs:103:62
    |
103 | #[doc = concat!("[`exit`]: ", include_str!(concat!("../../", env!("CARGO_PKG_NAME"), "/primitive_docs/process_exit.md")))]
    |                                                              ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)
