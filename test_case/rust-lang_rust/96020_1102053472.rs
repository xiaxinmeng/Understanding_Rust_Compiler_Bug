plain
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'workspaces::relative_rustc' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
error: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2430/lib/./foo -vV` (never executed)
Caused by:
  Text file busy (os error 26)
', src/tools/cargo/tests/testsuite/workspaces.rs:2147:42

---
test result: FAILED. 2456 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out; finished in 114.23s

error: test failed, to rerun pass '--test testsuite'
Build completed unsuccessfully in 0:24:15
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
