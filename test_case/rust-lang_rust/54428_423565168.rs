plain
[00:54:05] ....................................................................................................
[00:54:08] .....................................................i..............................................
[00:54:10] ....................................................................................................
[00:54:14] ....................................................................................................
[00:54:16] .iiiiiiiii..........................................................................................
[00:54:22] ....................................................................................................
[00:54:26] .....................................................................................i..............
[00:54:28] ....................................................................................................
[00:54:31] ........................................i.i..ii.....................................................
---
[01:09:46]     Finished release [optimized] target(s) in 44.55s
[01:09:46]      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/alloc-e8d35b7f9eb87295
[01:09:46] 
[01:09:46] running 101 tests
[01:09:46] .............F...................................................................................... 100/101
[01:09:46] failures:
[01:09:46] 
[01:09:46] 
[01:09:46] ---- boxed_test::unsafe_future_obj_into_raw_pinned stdout ----
[01:09:46] thread 'boxed_test::unsafe_future_obj_into_raw_pinned' panicked at 'pinned value should not be dropped', liballoc/boxed_test.rs:179:5
[01:09:46] 
[01:09:46] 
[01:09:46] failures:
[01:09:46] failures:
[01:09:46]     boxed_test::unsafe_future_obj_into_raw_pinned
[01:09:46] test result: FAILED. 100 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:09:46] 
[01:09:46] error: test failed, to rerun pass '--lib'
[01:09:46] 
[01:09:46] 
[01:09:46] 
[01:09:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:09:46] 
[01:09:46] 
[01:09:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:46] Build completed unsuccessfully in 0:24:20
[01:09:46] Build completed unsuccessfully in 0:24:20
[01:09:46] Makefile:58: recipe for target 'check' failed
[01:09:46] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b4c54f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:097cb336:start=1537542337101640836,finish=1537542337106427249,duration=4786413
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27ecfa78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\
