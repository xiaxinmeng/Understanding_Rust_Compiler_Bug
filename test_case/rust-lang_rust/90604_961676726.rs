plain
   Compiling rustc-demangle v0.1.21
error[E0658]: use of unstable library feature 'inherent_ascii_escape'
   --> library/alloc/src/slice.rs:112:9
    |
112 | pub use core::slice::EscapeAscii;
    |
    = note: see issue #77174 <https://github.com/rust-lang/rust/issues/77174> for more information
    = note: see issue #77174 <https://github.com/rust-lang/rust/issues/77174> for more information
    = help: add `#![feature(inherent_ascii_escape)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
