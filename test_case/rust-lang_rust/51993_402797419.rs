plain

[00:04:41] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:41] tidy error: /checkout/src/librustc_passes/rvalue_promotion.rs:518: trailing whitespace
[00:04:43] some tidy checks failed
[00:04:43] 
[00:04:43] 
[00:04:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:43] 
[00:04:43] 
[00:04:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:43] Build completed unsuccessfully in 0:01:44
[00:04:43] Build completed unsuccessfully in 0:01:44
[00:04:43] Makefile:79: recipe for target 'tidy' failed
[00:04:43] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11453bb0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02d1df46:start=1530811460769522731,finish=1530811460776117527,duration=6594796
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04bde3f5
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b02b38
$ dmesg | grep -i kill
