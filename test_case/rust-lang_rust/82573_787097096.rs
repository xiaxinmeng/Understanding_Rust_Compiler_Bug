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
test header::tests::debugger ... ok

failures:

---- tests::no stdout ----
thread 'tests::no' panicked at 'assertion failed: `(left == right)`
 right: `2`', src/tools/compiletest/src/tests.rs:75:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


