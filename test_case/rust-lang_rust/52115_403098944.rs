plain

[00:04:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:23] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/liveness.rs:79: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/librustc_mir/util/liveness.rs:345: line longer than 100 chars
[00:04:24] some tidy checks failed
[00:04:24] 
[00:04:24] 
[00:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:24] 
[00:04:24] 
[00:04:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:24] Build completed unsuccessfully in 0:00:47
[00:04:24] Build completed unsuccessfully in 0:00:47
[00:04:24] make: *** [tidy] Error 1
[00:04:24] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c73c773
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:34725414:start=1530898061462538636,finish=1530898061469979993,duration=7441357
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14902ac4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16102546
$ dmesg | grep -i kill
