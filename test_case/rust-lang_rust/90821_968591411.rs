
failures:

---- workspaces::relative_rustc stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'workspaces::relative_rustc' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
error: process exited with code 101 (expected 0)
--- stdout

--- stderr
error: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2233/lib/./foo -vV` (never executed)

Caused by:
  Text file busy (os error 26)
', src/tools/cargo/tests/testsuite/workspaces.rs:2142:42
