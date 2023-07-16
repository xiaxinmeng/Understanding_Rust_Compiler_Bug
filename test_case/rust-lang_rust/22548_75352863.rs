
---- [run-pass] run-pass/unwind-unique.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/unwind-unique.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-pass/unwind-unique.rs:18
thread '<main>' panicked at 'child thread None panicked', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/libstd/thread.rs:690

------------------------------------------

thread '[run-pass] run-pass/unwind-unique.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/compiletest/runtest.rs:1466


---- [run-pass] run-pass/unwind-resource.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/test/run-pass/unwind-resource.stage2-x86_64-unknown-linux-gnu 
stdout:
------------------------------------------
hiiiiiiiii
Hello!
About to send!
Sent!

------------------------------------------
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-pass/unwind-resource.rs:35
thread '<main>' panicked at 'child thread None panicked', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/libstd/thread.rs:690

------------------------------------------

thread '[run-pass] run-pass/unwind-resource.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/compiletest/runtest.rs:1466



failures:
    [run-pass] run-pass/unwind-resource.rs
    [run-pass] run-pass/unwind-unique.rs
