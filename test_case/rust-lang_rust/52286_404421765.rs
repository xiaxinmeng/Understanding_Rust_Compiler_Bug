plain

[00:04:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:00] tidy error: /checkout/src/librustc_errors/lib.rs:342: line longer than 100 chars
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:00:50
[00:04:02] Build completed unsuccessfully in 0:00:50
[00:04:02] make: *** [tidy] Error 1
[00:04:02] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21971a0f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01eccd38:start=1531381193036207499,finish=1531381193043976366,duration=7768867
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b4f1f70
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a1774d3
$ dmesg | grep -i kill
