
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
  --> external/crate_universe_dependencies__atty__0_2_14/src/lib.rs:21:1
   |
21 | extern crate libc;
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
