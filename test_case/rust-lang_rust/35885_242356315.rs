

failures:

---- [run-pass] run-pass/fds-are-cloexec.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/fds-are-cloexec.stage2-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 2, message: "No such file or directory" } }', ../src/libcore/result.rs:789
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/fds-are-cloexec.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/tools/compiletest/src/runtest.rs:2350
note: Run with `RUST_BACKTRACE=1` for a backtrace.
