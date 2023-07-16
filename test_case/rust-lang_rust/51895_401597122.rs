plain

[00:04:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:47] tidy error: /checkout/src/librustc/traits/object_safety.rs:390: line longer than 100 chars
[00:04:48] some tidy checks failed
[00:04:48] 
[00:04:48] 
[00:04:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:48] 
[00:04:48] 
[00:04:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:48] Build completed unsuccessfully in 0:01:37
[00:04:48] Build completed unsuccessfully in 0:01:37
[00:04:48] make: *** [tidy] Error 1
[00:04:48] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d3b528
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01337156:start=1530439448091204368,finish=1530439448098068594,duration=6864226
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00149e7a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3226499a
$ dmesg | grep -i kill
