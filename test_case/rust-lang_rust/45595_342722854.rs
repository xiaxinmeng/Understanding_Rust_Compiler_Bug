
[00:56:32] failures:
[00:56:32] 
[00:56:32] ---- feature_45_relative_cwd stdout ----
[00:56:32] 	thread 'feature_45_relative_cwd' panicked at 'assertion failed: `(left == right)`
[00:56:32]   left: `["bar/test", "baz/bar/test", "baz/foo", "baz/test", "foo", "test"]`,
[00:56:32]  right: `["bar/test", "baz/bar/test", "baz/baz/bar/test", "baz/foo", "baz/test", "foo", "test"]`', tests/tests.rs:1033:4
[00:56:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:32] 
[00:56:32] 
[00:56:32] failures:
[00:56:32]     feature_45_relative_cwd
[00:56:32] 
[00:56:32] test result: FAILED. 101 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:32] 
[00:56:32] error: test failed, to rerun pass '--test integration'
[00:56:32] thread 'main' panicked at 'tests failed for https://github.com/BurntSushi/ripgrep', /checkout/src/tools/cargotest/main.rs:100:8
[00:56:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
