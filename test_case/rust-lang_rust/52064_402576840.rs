plain

[00:04:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:48] tidy error: /checkout/src/libcore/mem.rs:231: trailing whitespace
[00:04:49] some tidy checks failed
[00:04:49] 
[00:04:49] 
[00:04:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:49] 
[00:04:49] 
[00:04:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:49] Build completed unsuccessfully in 0:01:44
[00:04:49] Build completed unsuccessfully in 0:01:44
[00:04:49] make: *** [tidy] Error 1
[00:04:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:30c63490
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02723ba4:start=1530750794457176390,finish=1530750794464305509,duration=7129119
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b6026ae
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b37dff
$ dmesg | grep -i kill
