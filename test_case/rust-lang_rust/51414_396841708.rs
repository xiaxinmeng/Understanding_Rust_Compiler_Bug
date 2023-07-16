plain

[00:05:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:15] tidy error: /checkout/src/test/run-pass/impl-trait/bounds_regression.rs: missing trailing newline
[00:05:16] some tidy checks failed
[00:05:16] 
[00:05:16] 
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:16] 
[00:05:16] 
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:16] Build completed unsuccessfully in 0:01:42
[00:05:16] Build completed unsuccessfully in 0:01:42
[00:05:16] make: *** [tidy] Error 1
[00:05:16] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e15b7d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10a3c5c4:start=1528874427842501595,finish=1528874427849687889,duration=7186294
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f23038
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:127cb460
$ dmesg | grep -i kill
