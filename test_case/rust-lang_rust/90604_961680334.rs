
error[E0658]: use of unstable library feature 'inherent_ascii_escape'
   --> library/alloc/src/slice.rs:112:9
    |
112 | pub use core::slice::EscapeAscii;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: see issue #77174 <https://github.com/rust-lang/rust/issues/77174> for more information
    = help: add `#![feature(inherent_ascii_escape)]` to the crate attributes to enable
