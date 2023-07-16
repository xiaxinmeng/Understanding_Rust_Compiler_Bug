plain
Resolving deltas: 100% (613138/613138), completed with 4854 local objects.
---
[00:00:57] configure: rust.quiet-tests     := True
---
[00:27:49] error[E0259]: the name `compiler_builtins` is defined multiple times
[00:27:49]    --> libstd/lib.rs:377:1
[00:27:49]     |
[00:27:49] 377 | extern crate compiler_builtins;
[00:27:49]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `compiler_builtins` reimported here
[00:27:49]     |
[00:27:49]     = note: `compiler_builtins` must be defined only once in the type namespace of this module
[00:27:49] help: You can use `as` to change the binding name of the import
[00:27:49]     |
[00:27:49] 377 | extern crate compiler_builtins as other_compiler_builtins;
