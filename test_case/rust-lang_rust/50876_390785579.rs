plain
[01:34:21] test workspaces::workspace_with_transitive_dev_deps ... ok
[01:34:21] 
[01:34:21] failures:
[01:34:21] 
[01:34:21] ---- freshness::changing_bin_features_caches_targets stdout ----
[01:34:21] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:34:21] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t536/foo/target/debug/off1`
[01:34:21] thread 'freshness::changing_bin_features_caches_targets' panicked at '
[01:34:21] Expected: execs
[01:34:21]     but: could not exec process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t536/foo/target/debug/off1`: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t536/foo/target/debug/off1` (never executed)
[01:34:21] caused by: could not execute process `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t536/foo/target/debug/off1` (never executed)
[01:34:21] caused by: Text file busy (os error 26)', tools/cargo/tests/testsuite/hamcrest.rs:13:9
[01:34:21] 
[01:34:21] 
[01:34:21] failures:
[01:34:21]     freshness::changing_bin_features_caches_targets
---
[01:34:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml"
[01:34:21] expected success, got: exit code: 101
[01:34:21] 
[01:34:21] 
[01:34:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:34:21] Build completed unsuccessfully in 0:32:31
[01:34:21] Makefile:60: recipe for target 'check-aux' failed
[01:34:21] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13c0c644
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
