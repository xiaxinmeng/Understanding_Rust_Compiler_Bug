
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
 --> t.rs:1:1
  |
1 | extern crate log;
  | ^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/27812
