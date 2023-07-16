
error: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, and unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
  --> src\tools\compiletest\src\main.rs:21:1
   |
21 | extern crate getopts;
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(rustc_private)] to the crate attributes to enable
