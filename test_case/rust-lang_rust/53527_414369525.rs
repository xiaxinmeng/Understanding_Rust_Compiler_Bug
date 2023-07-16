plain
[01:05:16]     Finished release [optimized] target(s) in 7.63s
[01:05:16]      Running build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/test-56e9d567d6eb6f83
[01:05:16] 
[01:05:16] running 38 tests
[01:05:16] ..........................F...........
[01:05:16] 
[01:05:16] ---- tests::exact_filter_match stdout ----
[01:05:16] thread 'tests::exact_filter_match' panicked at 'assertion failed: `(left == right)`
[01:05:16]   left: `4`,
---
[01:05:16] 
[01:05:16] error: test failed, to rerun pass '--lib'
[01:05:16] 
[01:05:16] 
[01:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:05:16] 
[01:05:16] 
[01:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:16] Build completed unsuccessfully in 0:18:22
[01:05:16] Build completed unsuccessfully in 0:18:22
[01:05:16] make: *** [check] Error 1
[01:05:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c1177fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
