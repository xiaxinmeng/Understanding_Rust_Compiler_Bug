plain

[00:04:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:08] tidy error: /checkout/src/test/compile-fail-fulldeps/proc-macro/proc-macro-custom-attr-mutex.rs:14: line longer than 100 chars
[00:04:09] some tidy checks failed
[00:04:09] 
[00:04:09] 
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:09] 
[00:04:09] 
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:09] Build completed unsuccessfully in 0:00:52
[00:04:09] Build completed unsuccessfully in 0:00:52
[00:04:09] Makefile:79: recipe for target 'tidy' failed
[00:04:09] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0185e2b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06e74889:start=1531256738545308883,finish=1531256738552642072,duration=7333189
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ccab40e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:033a1448
$ dmesg | grep -i kill
