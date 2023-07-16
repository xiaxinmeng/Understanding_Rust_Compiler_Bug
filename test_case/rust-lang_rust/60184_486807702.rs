text
---- [run-pass] run-pass/thinlto/thin-lto-inlines.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/home/jistone/rust/rust/build-llvm-i686/build/i686-unknown-linux-gnu/test/run-pass/thinlto/thin-lto-inlines/a"
stdout:
------------------------------------------
3 3

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `233`,
 right: `184`', /home/jistone/rust/rust/src/test/run-pass/thinlto/thin-lto-inlines.rs:28:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

------------------------------------------


---- [run-pass] run-pass/thinlto/thin-lto-inlines2.rs stdout ----

error: test run failed!
status: exit code: 101
command: "/home/jistone/rust/rust/build-llvm-i686/build/i686-unknown-linux-gnu/test/run-pass/thinlto/thin-lto-inlines2/a"
stdout:
------------------------------------------
3 3

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `83`,
 right: `184`', /home/jistone/rust/rust/src/test/run-pass/thinlto/thin-lto-inlines2.rs:26:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

------------------------------------------



failures:
    [run-pass] run-pass/thinlto/thin-lto-inlines.rs
    [run-pass] run-pass/thinlto/thin-lto-inlines2.rs

test result: FAILED. 2945 passed; 2 failed; 9 ignored; 0 measured; 0 filtered out
