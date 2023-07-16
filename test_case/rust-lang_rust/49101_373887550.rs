
   Compiling std_unicode v0.0.0 (file:///nobackup/rust/src/libstd_unicode)
error: this feature has been stable since 1.26.0. Attribute no longer needed
  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs:15:12
   |
15 | #![feature(i128_type)]
   |            ^^^^^^^^^
   |
note: lint level defined here
  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs:1:31
   |
1  | #![cfg_attr(not(stage0), deny(warnings))]
   |                               ^^^^^^^^
   = note: #[deny(stable_features)] implied by #[deny(warnings)]

error: aborting due to previous error

error: Could not compile `compiler_builtins`.
