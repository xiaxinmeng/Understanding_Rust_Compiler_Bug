plain
test test::test_workspaces_cwd ... ok

failures:

---- fix::fix_with_run_cargo_in_proc_macros stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
thread 'fix::fix_with_run_cargo_in_proc_macros' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
error: process exited with code 101 (expected 0)

--- stderr
   Compiling foo v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t915/foo)
error: proc macro panicked
error: proc macro panicked
 --> src/bin/main.rs:5:21
  |
5 |                     foo!("bar")
  |
  |
  = help: message: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
error: could not compile `foo` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
', src/tools/cargo/tests/testsuite/fix.rs:1746:10
---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:22:59
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
