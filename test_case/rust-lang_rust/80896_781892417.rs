text
error[E0658]: linking to associated items of raw pointers is experimental
  --> /home/jhpratt/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/metadata.rs:51:23
   |
51 | /// [`to_raw_parts`]: *const::to_raw_parts
   |                       ^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #80896 <https://github.com/rust-lang/rust/issues/80896> for more information
   = help: add `#![feature(intra_doc_pointers)]` to the crate attributes to enable
   = note: rustdoc does not allow disambiguating between `*const` and `*mut`, and pointers are unstable until it does
