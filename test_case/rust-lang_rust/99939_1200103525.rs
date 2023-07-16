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
.........F.........................i......................
failures:

---- tests::sort_tests stdout ----
thread 'tests::sort_tests' panicked at 'assertion failed: `(left == right)`
  left: `"isize::test_pow"`,
 right: `"sha1::test"`', library/test/src/tests.rs:626:9


failures:
    tests::sort_tests
    tests::sort_tests

test result: FAILED. 56 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.26s

error: test failed, to rerun pass '-p test --lib'
Build completed unsuccessfully in 0:01:40
