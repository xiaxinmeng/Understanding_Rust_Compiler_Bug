plain

[00:05:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:23] tidy error: /checkout/src/bootstrap/dist.rs:1306: line longer than 100 chars
[00:05:23] tidy error: /checkout/src/tools/build-manifest/src/main.rs:263: line longer than 100 chars
[00:05:24] some tidy checks failed
[00:05:24] 
[00:05:24] 
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:24] 
[00:05:24] 
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:24] Build completed unsuccessfully in 0:01:54
[00:05:24] Build completed unsuccessfully in 0:01:54
[00:05:24] Makefile:79: recipe for target 'tidy' failed
[00:05:24] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0259dc18
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03a58e20:start=1529748763665147738,finish=1529748763674232340,duration=9084602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2df1fb48
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16af3ac0
$ dmesg | grep -i kill
