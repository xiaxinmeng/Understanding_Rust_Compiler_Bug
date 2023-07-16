plain
[00:03:59]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:59] [RUSTC-TIMING] panic_unwind test:false 0.263
[00:04:14] [RUSTC-TIMING] std test:false 14.726
[00:04:14]     Finished release [optimized] target(s) in 49.18s
[00:04:14] thread 'main' panicked at 'no output generated for "libstd" "so"', bootstrap/compile.rs:1113:21
[00:04:14] travis_fold:end:stage0-std

[00:04:14] travis_time:end:stage0-std:start=1529947198382166086,finish=1529947247597333601,duration=49215167515


[00:04:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:50
[00:04:14] make: *** [tidy] Error 1
[00:04:14] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:014a11c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e914992:start=1529947248239357016,finish=1529947248247600895,duration=8243879
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:073b1aa3
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04a75784
$ dmesg | grep -i kill
