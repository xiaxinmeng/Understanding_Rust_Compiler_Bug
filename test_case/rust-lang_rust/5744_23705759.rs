
[...]
error: test run failed!
command: i686-unknown-linux-gnu/test/run-pass/extern-pass-TwoU8s.stage2-i686-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
task <unnamed> failed at 'assertion failed: `(left == right) && (right == left)` (left: `TwoU8s{one: 22u8, two: 23u8}`, right: `TwoU8s{one: 22u8, two: 0u8}`)', /home/rustbuild/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/test/run-pass/extern-pass-TwoU8s.rs:28

------------------------------------------

task <unnamed> failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/compiletest/runtest.rs:778
test [run-pass] run-pass/extern-pass-TwoU8s.rs ... FAILED
test [run-pass] run-pass/extern-pass-TwoU64s-ref.rs ... ok
test [run-pass] run-pass/extoption_env-not-defined.rs ... ok

error: test run failed!
command: i686-unknown-linux-gnu/test/run-pass/extern-pass-TwoU16s.stage2-i686-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
task <unnamed> failed at 'assertion failed: `(left == right) && (right == left)` (left: `TwoU16s{one: 22u16, two: 23u16}`, right: `TwoU16s{one: 22u16, two: 0u16}`)', /home/rustbuild/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/test/run-pass/extern-pass-TwoU16s.rs:28

------------------------------------------

task <unnamed> failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/auto-linux-32-nopt-t/build/src/compiletest/runtest.rs:778
test [run-pass] run-pass/extern-pass-TwoU16s.rs ... FAILED
[...]
