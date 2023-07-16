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
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:190:9
......................................................
failures:

---- f32::tests::test_gamma stdout ----
thread 'f32::tests::test_gamma' panicked at 'assertion failed: `(left == right)`
  left: `2.0000002`,
 right: `2.0`', library/std/src/f32/tests.rs:558:5

failures:
    f32::tests::test_gamma


test result: FAILED. 932 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.28s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:01:40
