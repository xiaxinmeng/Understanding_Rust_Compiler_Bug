
failures:

---- [compile-fail] compile-fail/issue-17546.rs stdout ----

error: expected error: on line 22 not found: found value `foo::MyEnum::NoResult` used as a type
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-17546.rs -L i686-unknown-linux-gnu/test/compile-fail/ --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/compile-fail/issue-17546.stage2-i686-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o i686-unknown-linux-gnu/test/compile-fail/issue-17546.stage2-i686-unknown-linux-gnu --cfg rtopt -O -L i686-unknown-linux-gnu/rt
stdout:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: probe.index() != ib + size + 1', ../src/libstd/collections/hash/map.rs:1204



------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace

------------------------------------------

thread '[compile-fail] compile-fail/issue-17546.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/compiletest/runtest.rs:1501



failures:
    [compile-fail] compile-fail/issue-17546.rs
