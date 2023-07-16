plain

[00:04:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:54] tidy error: /checkout/src/librustc/ty/structural_impls.rs:995: line longer than 100 chars
[00:04:54] tidy error: /checkout/src/librustc/ty/structural_impls.rs:1020: line longer than 100 chars
[00:04:55] some tidy checks failed
[00:04:55] 
[00:04:55] 
[00:04:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:55] 
[00:04:55] 
[00:04:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:55] Build completed unsuccessfully in 0:01:58
[00:04:55] Build completed unsuccessfully in 0:01:58
[00:04:55] Makefile:79: recipe for target 'tidy' failed
[00:04:55] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09a0058d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:14d8ef80:start=1530262108446766476,finish=1530262108452934981,duration=6168505
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a7ab24b
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2e939b2c
$ dmesg | grep -i kill
