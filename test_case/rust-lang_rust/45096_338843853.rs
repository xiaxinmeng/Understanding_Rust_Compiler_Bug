
[01:35:39] test test::test_bin_lib_project has been running for over 60 seconds
[01:39:59] test test::test_bin_lib_project ... FAILED
[01:39:59]
[01:39:59] failures:
[01:39:59]
[01:39:59] ---- test::test_bin_lib_project stdout ----
[01:39:59] thread 'test::test_bin_lib_project' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:189:12
[01:39:59]
[01:39:59]
[01:39:59] failures:
[01:39:59] test::test_bin_lib_project
[01:39:59]
[01:39:59] test result: FAILED. 26 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:39:59]
[01:39:59] error: test failed, to rerun pass '--bin rls'
[01:39:59]
[01:39:59]
[01:39:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml"
[01:39:59] expected success, got: exit code: 101
[01:39:59]
[01:39:59]
[01:39:59] You can disable the tool in `src/tools/toolstate.toml`
