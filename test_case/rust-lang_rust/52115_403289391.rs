plain

[00:03:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:48] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/liveness.rs:79: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/librustc_mir/util/liveness.rs:117: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/librustc_mir/util/liveness.rs:141: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/librustc_mir/util/liveness.rs:372: line longer than 100 chars
[00:03:50] some tidy checks failed
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:50
[00:03:50] Build completed unsuccessfully in 0:00:50
[00:03:50] Makefile:79: recipe for target 'tidy' failed
[00:03:50] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0610a5f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1d84fdba:start=1531057878711266136,finish=1531057878718215191,duration=6949055
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1518100f
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09756d00
$ dmesg | grep -i kill
