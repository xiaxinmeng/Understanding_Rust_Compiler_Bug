plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
---- src/slice/mod.rs - slice::[T]::aligned_subslice (line 3617) stdout ----
error[E0658]: use of unstable library feature 'slice_align_to_ish'
 --> src/slice/mod.rs:3620:22
  |
6 |     let ints = bytes.aligned_subslice(std::mem::align_of::<u32>());
  |
  |
  = help: add `#![feature(slice_align_to_ish)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::aligned_subslice_mut (line 3656) stdout ----
error[E0658]: use of unstable library feature 'slice_align_to_ish'
 --> src/slice/mod.rs:3659:22
  |
6 |     let ints = bytes.aligned_subslice_mut(std::mem::align_of::<u32>());
  |
  |
  = help: add `#![feature(slice_align_to_ish)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::transmute_elements (line 3696) stdout ----
error[E0425]: cannot find value `bytes` in this scope
 --> src/slice/mod.rs:3699:24
  |
6 |     let smaller_ints = bytes.transmute_elements::<u16>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- src/slice/mod.rs - slice::[T]::transmute_elements_mut (line 3737) stdout ----
error[E0425]: cannot find value `bytes` in this scope
 --> src/slice/mod.rs:3740:24
  |
6 |     let smaller_ints = bytes.transmute_elements_mut::<u16>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
---
    src/slice/mod.rs - slice::[T]::transmute_elements_mut (line 3737)

test result: FAILED. 3933 passed; 4 failed; 48 ignored; 0 measured; 0 filtered out; finished in 52.45s

error: doctest failed, to rerun pass `-p core --doc`
