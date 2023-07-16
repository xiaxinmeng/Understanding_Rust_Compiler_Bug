
   Compiling rustc_term v0.0.1
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
 --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc_term-0.0.1\win.rs:5:1
  |
5 | extern crate libc;
  | ^^^^^^^^^^^^^^^^^^
  |
  = help: add #![feature(rustc_private)] to the crate attributes to enable
