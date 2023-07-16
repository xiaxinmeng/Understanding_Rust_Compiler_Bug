plain

[00:04:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:14] tidy error: /checkout/src/libsyntax/ext/base.rs:258: line longer than 100 chars
[00:04:15] some tidy checks failed
[00:04:15] 
[00:04:15] 
[00:04:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:15] 
[00:04:15] 
[00:04:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:15] Build completed unsuccessfully in 0:00:50
[00:04:15] Build completed unsuccessfully in 0:00:50
[00:04:15] make: *** [tidy] Error 1
[00:04:15] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16764448
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01519669:start=1531250122680748534,finish=1531250122693419710,duration=12671176
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fb71c23
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04f95066
$ dmesg | grep -i kill
