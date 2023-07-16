


---- [pretty] run-pass/bool-not.rs stdout ----

error: pretty-printed source (expanded) does not typecheck
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --crate-type=lib --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/ -L x86_64-unknown-linux-gnu/test/run-pass/bool-not.stage2-x86_64-unknown-linux-gnulibaux --cfg rtopt --cfg debug -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------
thread 'rustc' panicked at 'internal error: parse_def_id: crate number expected, found [48]', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-opt/build/src/librustc/metadata/tydecode.rs:731



------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace

------------------------------------------

thread '[pretty] run-pass/bool-not.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-opt/build/src/compiletest/runtest.rs:1482



failures:
    [pretty] run-pass/bool-not.rs

test result: FAILED. 1944 passed; 1 failed; 74 ignored; 0 measured
