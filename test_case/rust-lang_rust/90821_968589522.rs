plain

---- workspaces::relative_rustc stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'workspaces::relative_rustc' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
error: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2233/lib/./foo -vV` (never executed)
Caused by:
Caused by:
  Text file busy (os error 26)


failures:
    workspaces::relative_rustc
---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:23:58
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
