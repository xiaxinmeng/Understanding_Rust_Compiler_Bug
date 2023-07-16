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
.....................................................
failures:

---- sys::unix::os_str::tests::debug stdout ----
thread 'sys::unix::os_str::tests::debug' panicked at 'assertion failed: `(left == right)`
  left: `"\"Hello\\xc0\\x80 There\\xe6\\x83 Goodbye\\u{10d4ea}\""`,
 right: `"\"Hello\\xC0\\x80 There\\xE6\\x83 Goodbye\\u{10d4ea}\""`', library/std/src/sys/unix/../unix/os_str/tests.rs:14:5

failures:
    sys::unix::os_str::tests::debug


test result: FAILED. 931 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.32s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:01:38
