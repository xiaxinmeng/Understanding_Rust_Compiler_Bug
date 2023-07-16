plain

[00:05:02] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:02] tidy error: /checkout/src/librustc_typeck/check/mod.rs:5062: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/librustc_typeck/check/method/confirm.rs:319: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/librustc_typeck/check/method/confirm.rs:327: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/librustc_typeck/check/method/confirm.rs:331: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/librustc_typeck/check/method/confirm.rs:348: line longer than 100 chars
[00:05:04] some tidy checks failed
[00:05:04] 
[00:05:04] 
[00:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:04] 
[00:05:04] 
[00:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:04] Build completed unsuccessfully in 0:01:51
[00:05:04] Build completed unsuccessfully in 0:01:51
[00:05:04] Makefile:79: recipe for target 'tidy' failed
[00:05:04] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0797a1c6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1b1ef8a4:start=1530221099467027812,finish=1530221099473312487,duration=6284675
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c85a784
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:072c5478
$ dmesg | grep -i kill
