plain

[00:04:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:50] tidy error: /checkout/src/test/ui/rfc-2093-infer-outlives/cross-crate.rs:16: line longer than 100 chars
[00:04:52] some tidy checks failed
[00:04:52] 
[00:04:52] 
[00:04:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:52] 
[00:04:52] 
[00:04:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:52] Build completed unsuccessfully in 0:01:32
[00:04:52] Build completed unsuccessfully in 0:01:32
[00:04:52] Makefile:79: recipe for target 'tidy' failed
[00:04:52] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:083e07d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00588fab:start=1530585318893507874,finish=1530585318902973932,duration=9466058
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0beba12e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:054885f3
$ dmesg | grep -i kill
