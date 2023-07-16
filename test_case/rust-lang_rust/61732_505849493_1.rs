
error[E0578]: cannot find module `mod` in module `crate`
 --> main.rs:1:15
  |
1 | pub(in crate::r#mod) fn foo() {}
  |               ^^^^^ not found in `crate`

error: Compilation failed, aborting rustdoc
