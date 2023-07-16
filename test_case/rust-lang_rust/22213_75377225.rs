

failures:

---- [run-pass-valgrind] run-pass-valgrind/dst-dtor-1.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass-valgrind/dst-dtor-1.stage2-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: DROP_RAN', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/run-pass-valgrind/dst-dtor-1.rs:34

------------------------------------------

thread '[run-pass-valgrind] run-pass-valgrind/dst-dtor-1.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/compiletest/runtest.rs:1466


---- [run-pass-valgrind] run-pass-valgrind/dst-dtor-2.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass-valgrind/dst-dtor-2.stage2-x86_64-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: DROP_RAN == 3', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/run-pass-valgrind/dst-dtor-2.rs:31

------------------------------------------

thread '[run-pass-valgrind] run-pass-valgrind/dst-dtor-2.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/compiletest/runtest.rs:1466

