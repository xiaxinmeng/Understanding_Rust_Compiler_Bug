
[00:21:43] error: unused attribute
[00:21:43]  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs:5:1
[00:21:43]   |
[00:21:43] 5 | #![crate_type = "rlib"]
[00:21:43]   | ^^^^^^^^^^^^^^^^^^^^^^^
[00:21:43]   |
[00:21:43] note: lint level defined here
[00:21:43]  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs:1:31
[00:21:43]   |
[00:21:43] 1 | #![cfg_attr(not(stage0), deny(warnings))]
[00:21:43]   |                               ^^^^^^^^
[00:21:43]   = note: #[deny(unused_attributes)] implied by #[deny(warnings)]
[00:21:43] 
[00:21:43] error: crate-level attribute should be in the root module
[00:21:43]  --> rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs:5:1
[00:21:43]   |
[00:21:43] 5 | #![crate_type = "rlib"]
[00:21:43]   | ^^^^^^^^^^^^^^^^^^^^^^^
[00:21:43] 
[00:21:43] error: aborting due to 2 previous errors
[00:21:43] 
[00:21:43] error: Could not compile `compiler_builtins`.
