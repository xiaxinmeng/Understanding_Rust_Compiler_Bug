
---- fix::fix_deny_warnings_but_not_others stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
thread 'fix::fix_deny_warnings_but_not_others' panicked at 'assertion failed: p.read_file(\"src/lib.rs\").contains(\"fn bar() {}\")', src/tools/cargo/tests/testsuite/fix.rs:572:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
