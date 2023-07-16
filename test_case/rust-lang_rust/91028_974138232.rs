
 % cargo +1.56 build
...
note: `Option` could also refer to the enum defined here

% rustup component add --toolchain 1.56 rust-src

% cargo +1.56 build
...
note: `Option` could also refer to the enum defined here
   --> /Users/shep/.rustup/toolchains/1.56-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/prelude/mod.rs:133:13
    |
133 |     pub use core::prelude::rust_2021::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
