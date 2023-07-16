
error[E0463]: can't find crate for `nom`
 --> src/main.rs:1:1
  |
1 | extern crate nom;
  | ^^^^^^^^^^^^^^^^^ can't find crate
  note: nom exists in the sysroot, but this is an unstable location, use #![feature(rustc_private)] to access it.
        You probably want to add `nom` to Cargo.toml instead.
