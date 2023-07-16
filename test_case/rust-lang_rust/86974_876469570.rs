plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error[E0432]: unresolved import `rustc_span::with_default_session_globals`
 --> compiler/rustc_lint/src/tests.rs:2:18
  |
2 | use rustc_span::{with_default_session_globals, Symbol};
  |                  |
  |                  |
  |                  no `with_default_session_globals` in the root
  |                  help: a similar name exists in the module: `with_session_globals`
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
