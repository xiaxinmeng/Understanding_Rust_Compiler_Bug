plain

[00:05:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:10] tidy error: /checkout/src/librustc_mir/borrow_check/nll/constraint_set.rs:17: line longer than 100 chars
[00:05:10] tidy error: /checkout/src/librustc_mir/borrow_check/nll/constraint_set.rs:31: line longer than 100 chars
[00:05:10] tidy error: /checkout/src/librustc_mir/borrow_check/nll/constraint_set.rs: incorrect license
[00:05:11] some tidy checks failed
[00:05:11] 
[00:05:11] 
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:11] 
[00:05:11] 
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:11] Build completed unsuccessfully in 0:01:54
[00:05:11] Build completed unsuccessfully in 0:01:54
[00:05:11] Makefile:79: recipe for target 'tidy' failed
[00:05:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d1389d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04a65d5b:start=1530133238118435305,finish=1530133238124393049,duration=5957744
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0866b175
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036c4460
$ dmesg | grep -i kill
