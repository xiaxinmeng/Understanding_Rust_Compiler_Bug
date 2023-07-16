plain
[00:52:23] .............................................................................ii.....................
[00:53:11] .........................................i....................................................i.ii..
[00:53:21] ......................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:53:53] ..............................................................................
[00:54:10] ..iiiiiii...........................................................................................
[00:54:50] ....................................................................................................
[00:55:08] ...........................................................................
[00:55:08] test result: ok. 2956 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:55:08] 
---
[01:08:25] 
[01:08:25]      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-f9e3fba9f9b598c3
[01:08:25] 
[01:08:25] running 490 tests
[01:08:25] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-f9e3fba9f9b598c3 --quiet` (signal: 11, SIGSEGV: invalid memory reference)
[01:08:25] 
[01:08:25] 
[01:08:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:08:25] 
[01:08:25] 
[01:08:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:25] Build completed unsuccessfully in 0:26:50
[01:08:25] Build completed unsuccessfully in 0:26:50
[01:08:25] Makefile:58: recipe for target 'check' failed
[01:08:25] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12d935a2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
