quote
---- workspaces::relative_rustc stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
thread 'workspaces::relative_rustc' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout

--- stderr
error: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1927/lib/./foo -vV` (never executed)

Caused by:
  Text file busy (os error 26)
', src/tools/cargo/crates/cargo-test-support/src/lib.rs:832:13
