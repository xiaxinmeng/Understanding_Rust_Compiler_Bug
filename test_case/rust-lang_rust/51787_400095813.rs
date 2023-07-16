plain
[01:08:59] 
[01:08:59] running 2093 tests
[01:09:11] ....................................................................................................
[01:09:24] ....................................................................................................
[01:09:40] ........................................................................................F...........
[01:10:07] ....................................................................................................
[01:10:19] ....................................................................................................
[01:10:31] ....................................................................................................
[01:10:43] ....................................................................................................
---
[01:13:41] ---- iter/iterator.rs - iter::iterator::Iterator::map_into (line 495) stdout ----
[01:13:41] error[E0658]: use of unstable library feature 'move_into'
[01:13:41]  --> iter/iterator.rs:496:15
[01:13:41]   |
[01:13:41] 4 | (1i32..42i32).map_into().collect::<Vec<f64>>();
[01:13:41]   |
[01:13:41]   |
[01:13:41]   = help: add #![feature(move_into)] to the crate attributes to enable
[01:13:41] thread 'iter/iterator.rs - iter::iterator::Iterator::map_into (line 495)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:13:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:41] 
[01:13:41] 
---
[01:13:41] 
[01:13:41] error: test failed, to rerun pass '--doc'
[01:13:41] 
[01:13:41] 
[01:13:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:13:41] 
[01:13:41] 
[01:13:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:41] Build completed unsuccessfully in 0:31:03
[01:13:41] Build completed unsuccessfully in 0:31:03
[01:13:41] make: *** [check] Error 1
[01:13:41] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12638d0c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02243a34:start=1529960815829376522,finish=1529960815836011590,duration=6635068
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ce7f8b
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06804d70
$ dmesg | grep -i kill
