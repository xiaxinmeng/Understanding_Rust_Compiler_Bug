plain
error: process exited with code 101 (expected 0)
--- stdout

--- stderr
error: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2500/lib/./foo -vV` (never executed)
Caused by:
  Text file busy (os error 26)
', src/tools/cargo/tests/testsuite/workspaces.rs:2176:42

---
test result: FAILED. 2530 passed; 1 failed; 142 ignored; 0 measured; 0 filtered out; finished in 112.03s

error: test failed, to rerun pass `--test testsuite`
Build completed unsuccessfully in 0:26:16
make: *** [Makefile:44: check-aux] Error 1
