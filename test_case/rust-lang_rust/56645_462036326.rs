
[01:51:58] failures:
[01:51:58] 
[01:51:58] ---- fix::shows_warnings stdout ----
[01:51:58] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
[01:51:58] thread 'fix::shows_warnings' panicked at '
[01:51:58] Expected: execs
[01:51:58]     but: expected to find:
[01:51:58] [..]warning: unused import[..]
[01:51:58] 
[01:51:58] did not find in output:
[01:51:58]     Checking foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t615/foo)
[01:51:58]       Fixing src/lib.rs (1 fix)
[01:51:58]     Finished dev [unoptimized + debuginfo] target(s) in 0.66s
[01:51:58] ', src/tools/cargo/tests/testsuite/support/mod.rs:794:13
[01:51:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:51:58] 
[01:51:58] ---- fix::shows_warnings_on_second_run_without_changes stdout ----
[01:51:58] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs`
[01:51:58] thread 'fix::shows_warnings_on_second_run_without_changes' panicked at '
[01:51:58] Expected: execs
[01:51:58]     but: expected to find:
[01:51:58] [..]warning: unused import[..]
[01:51:58] 
[01:51:58] did not find in output:
[01:51:58]     Checking foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t616/foo)
[01:51:58]       Fixing src/lib.rs (1 fix)
[01:51:58]     Finished dev [unoptimized + debuginfo] target(s) in 0.77s
[01:51:58] ', src/tools/cargo/tests/testsuite/support/mod.rs:794:13
[01:51:58] 
[01:51:58] ---- fix::shows_warnings_on_second_run_without_changes_on_multiple_targets stdout ----
[01:51:58] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --allow-no-vcs --all-targets`
[01:51:58] thread 'fix::shows_warnings_on_second_run_without_changes_on_multiple_targets' panicked at '
[01:51:58] Expected: execs
[01:51:58]     but: expected to find:
[01:51:58]  --> examples/fooxample.rs:2:21
[01:51:58] 
[01:51:58] did not find in output:
[01:51:58]     Checking foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t617/foo)
[01:51:58]       Fixing src/lib.rs (1 fix)
[01:51:58]       Fixing tests/foo.rs (1 fix)
[01:51:58]       Fixing tests/bar.rs (1 fix)
[01:51:58]       Fixing src/main.rs (1 fix)
[01:51:58]       Fixing examples/fooxample.rs (1 fix)
[01:51:58]     Finished dev [unoptimized + debuginfo] target(s) in 1.89s
[01:51:58] ', src/tools/cargo/tests/testsuite/support/mod.rs:794:13
[01:51:58] 
[01:51:58] 
[01:51:58] failures:
[01:51:58]     fix::shows_warnings
[01:51:58]     fix::shows_warnings_on_second_run_without_changes
[01:51:58]     fix::shows_warnings_on_second_run_without_changes_on_multiple_targets
[01:51:58] 
[01:51:58] test result: FAILED. 1516 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
[01:51:58] 
[01:51:58] [0m[0m[1m[31merror:[0m test failed, to rerun pass '--test testsuite'
[01:51:58] 
