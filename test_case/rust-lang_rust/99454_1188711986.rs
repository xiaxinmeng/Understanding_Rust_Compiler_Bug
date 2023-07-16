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
---- src/ops/control_flow.rs - ops::control_flow::ControlFlow<B,C>::continue_value (line 204) stdout ----
error[E0308]: mismatched types
 --> src/ops/control_flow.rs:209:1
  |
8 | assert_eq!(ControlFlow::<String, i32>::Continue(3).break_value(), Some(3));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `String`, found integer
  = note: expected enum `Option<String>`
             found enum `Option<{integer}>`
             found enum `Option<{integer}>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
