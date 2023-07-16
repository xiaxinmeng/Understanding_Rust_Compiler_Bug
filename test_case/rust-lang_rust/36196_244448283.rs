
Compiling serialize v0.0.0 (file:///Users/abhishek/code/rust/src/libserialize)
error[E0460]: found possibly newer version of crate `std` which `log` depends on
  --> src/libserialize/lib.rs:41:14
   |
41 | #[macro_use] extern crate log;
   |              ^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: crate `std` path #1: /Users/abhishek/code/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-9a82330aa9f25cb1.rlib
   = note: crate `std` path #2: /Users/abhishek/code/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-9a82330aa9f25cb1.dylib
   = note: crate `log` path #1: /Users/abhishek/code/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblog-23e007e69714d505.dylib
   = note: crate `log` path #2: /Users/abhishek/code/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblog-23e007e69714d505.rlib

error: aborting due to previous error

error: Could not compile `serialize`.
