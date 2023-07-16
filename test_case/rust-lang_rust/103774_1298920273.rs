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
........................................................................................ 1496/1506
..........
failures:

---- any::dyn_type_name stdout ----
error: test failed, to rerun pass `-p core --test coretests`
thread 'any::dyn_type_name' panicked at 'assertion failed: `(left == right)`
  left: `"dyn core::ops::function::Fn(i32, i32) -> i32"`,
 right: `"dyn core::ops::function::Fn<(i32, i32)>+Output = i32"`', library/core/tests/any.rs:141:5


failures:
    any::dyn_type_name
