plain
[01:17:06] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:07]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:17:11] error[E0152]: duplicate lang item found: `begin_panic`.
[01:17:11]     |
[01:17:11]     |
[01:17:11] 402 | / pub fn begin_panic<M: Any + Send>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
[01:17:11] 403 | |     // Note that this should be the only allocation performed in this code path.
[01:17:11] 404 | |     // Currently this means that panic!() on OOM will invoke this code path,
[01:17:11] 405 | |     // but then again we're not really ready for panic on OOM anyway. If
[01:17:11] 437 | |     }
[01:17:11] 438 | | }
[01:17:11]     | |_^
[01:17:11]     |
[01:17:11]     |
[01:17:11]     = note: first defined in crate `std`.
[01:17:11] error: aborting due to previous error
[01:17:11] 
[01:17:11] For more information about this error, try `rustc --explain E0152`.
[01:17:11] error: Could not compile `std`.
[01:17:11] error: Could not compile `std`.
[01:17:11] 
[01:17:11] To learn more, run the command again with --verbose.
[01:17:11] 
[01:17:11] 
[01:17:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:17:11] 
[01:17:11] 
[01:17:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:11] Build completed unsuccessfully in 0:34:12
[01:17:11] Build completed unsuccessfully in 0:34:12
[01:17:11] Makefile:58: recipe for target 'check' failed
[01:17:11] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fc58b90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03730400:start=1530626213106057275,finish=1530626213113776165,duration=7718890
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd23154
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f39f84f
$ dmesg | grep -i kill
