plain

[00:04:40] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:41] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:100: line longer than 100 chars
[00:04:41] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:213: line longer than 100 chars
[00:04:41] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:225: line longer than 100 chars
[00:04:41] tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs: incorrect license
[00:04:42] some tidy checks failed
[00:04:42] 
[00:04:42] 
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:42] 
[00:04:42] 
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:42] Build completed unsuccessfully in 0:01:45
[00:04:42] Build completed unsuccessfully in 0:01:45
[00:04:42] make: *** [tidy] Error 1
[00:04:42] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25a07bb2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c6b1f55:start=1529754252697697590,finish=1529754252705203394,duration=7505804
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:046fc188
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04d6a78a
$ dmesg | grep -i kill
