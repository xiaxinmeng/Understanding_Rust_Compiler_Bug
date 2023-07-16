plain
[01:43:51] test workspaces::workspace_with_transitive_dev_deps ... ok
[01:43:51] 
[01:43:51] failures:
[01:43:51] 
[01:43:51] ---- freshness::changing_bin_features_caches_targets stdout ----
[01:43:51]  running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:43:51] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t532/foo/target/debug/off1`
[01:43:51] thread 'freshness::changing_bin_features_caches_targets' panicked at '
[01:43:51] Expected: execs
[01:43:51]     but: could not exec process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t532/foo/target/debug/off1`: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t532/foo/target/debug/off1` (never executed)
[01:43:51] caused by: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t532/foo/target/debug/off1` (never executed)
[01:43:51] caused by: Text file busy (os error 26)', tools/cargo/tests/testsuite/hamcrest.rs:13:9
[01:43:51] 
[01:43:51] 
[01:43:51] failures:
[01:43:51]     freshness::changing_bin_features_caches_targets
---
[01:43:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml"
[01:43:51] expected success, got: exit code: 101
[01:43:51] 
[01:43:51] 
[01:43:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:43:51] Build completed unsuccessfully in 0:36:16
[01:43:51] make: *** [check-aux] Error 1
[01:43:51] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0835b768
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
