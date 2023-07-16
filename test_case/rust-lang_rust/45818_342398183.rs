
[01:00:08] failures:
[01:00:08] 
[01:00:08] ---- regression_184 stdout ----
[01:00:08] 	thread 'regression_184' panicked at '
[01:00:08] 
[01:00:08] ==========
[01:00:08] command failed but expected success!
[01:00:08] 
[01:00:08] command: "/checkout/obj/build/ct/ripgrep/target/debug/deps/../rg" "test" "."
[01:00:08] cwd: /checkout/obj/build/ct/ripgrep/target/debug/deps/ripgrep-tests/regression_184/65
[01:00:08] 
[01:00:08] status: exit code: 1
[01:00:08] 
[01:00:08] stdout: 
[01:00:08] 
[01:00:08] stderr: No files were searched, which means ripgrep probably applied a filter you didn't expect. Try running again with --debug.
[01:00:08] 
[01:00:08] 
[01:00:08] ==========
[01:00:08] ', tests/workdir.rs:220:12
[01:00:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:08] 
[01:00:08] 
[01:00:08] failures:
[01:00:08]     regression_184
[01:00:08] 
[01:00:08] test result: FAILED. 101 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
