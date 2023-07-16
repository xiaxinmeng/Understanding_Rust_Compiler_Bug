plain
[01:13:15] 
[01:13:15] error: test failed, to rerun pass '--doc'
[01:13:15] 
[01:13:15] 
[01:13:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:13:15] 
[01:13:15] 
[01:13:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:15] Build completed unsuccessfully in 0:32:19
[01:13:15] Build completed unsuccessfully in 0:32:19
[01:13:15] Makefile:58: recipe for target 'check' failed
[01:13:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a402f21
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
