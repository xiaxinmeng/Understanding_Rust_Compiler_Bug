plain
test lto::cdylib_and_rlib ... ok

failures:

---- fix::fix_in_existing_repo_weird_ignore stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix`
thread 'fix::fix_in_existing_repo_weird_ignore' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
error: no VCS found for this package and `cargo fix` can potentially perform destructive changes; if you'd like to suppress this error pass `--allow-no-vcs`
', src/tools/cargo/tests/testsuite/fix.rs:1403:20


failures:
    fix::fix_in_existing_repo_weird_ignore
---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:24:36
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
