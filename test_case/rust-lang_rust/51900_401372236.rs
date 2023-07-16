plain

[00:04:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:52] tidy error: /checkout/src/librustc_mir/dataflow/mod.rs:193: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_mir/dataflow/mod.rs:241: line longer than 100 chars
[00:04:53] some tidy checks failed
[00:04:53] 
[00:04:53] 
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:53] 
[00:04:53] 
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:53] Build completed unsuccessfully in 0:01:46
[00:04:53] Build completed unsuccessfully in 0:01:46
[00:04:53] Makefile:79: recipe for target 'tidy' failed
[00:04:53] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1789215c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:256c7a5c:start=1530282582815116637,finish=1530282582820589203,duration=5472566
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13c87b8a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:040250cc
$ dmesg | grep -i kill
