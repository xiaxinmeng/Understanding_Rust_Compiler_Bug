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
---- src/ptr/mod.rs - ptr::drop_in_place (line 442) stdout ----
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/ptr/mod.rs:445:21
  |
6 |     let mut value = &mut *to_drop;
  |                     ^^^^^^^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/ptr/mod.rs - ptr::drop_in_place (line 442)

test result: FAILED. 3916 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 48.08s

error: doctest failed, to rerun pass `-p core --doc`
