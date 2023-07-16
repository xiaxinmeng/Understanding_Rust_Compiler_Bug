plain
[01:16:23] 
[01:16:23]      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-005c3aa196393c84
[01:16:23] 
[01:16:23] running 490 tests
[01:16:23] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-005c3aa196393c84 --quiet` (signal: 11, SIGSEGV: invalid memory reference)
[01:16:23] 
[01:16:23] 
[01:16:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:16:23] 
[01:16:23] 
[01:16:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:23] Build completed unsuccessfully in 0:29:55
[01:16:23] Build completed unsuccessfully in 0:29:55
[01:16:23] Makefile:58: recipe for target 'check' failed
[01:16:23] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a7667f5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
