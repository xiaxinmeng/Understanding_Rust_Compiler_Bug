

failures:

---- [run-pass] run-pass/command-before-exec.rs stdout ----

error: test run failed!
status: exit code: 101
command: i686-apple-darwin/test/run-pass/command-before-exec.stage2-i686-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: output.status.success()', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/command-before-exec.rs:64
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/command-before-exec.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1651
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/command-before-exec.rs

