plain

[00:04:33] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:33] tidy error: /checkout/src/librustc_mir/dataflow/mod.rs:193: line longer than 100 chars
[00:04:33] tidy error: /checkout/src/librustc_mir/dataflow/mod.rs:241: line longer than 100 chars
[00:04:33] tidy error: /checkout/src/librustc_mir/dataflow/mod.rs:254: line longer than 100 chars
[00:04:34] some tidy checks failed
[00:04:34] 
[00:04:34] 
[00:04:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:34] 
[00:04:34] 
[00:04:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:34] Build completed unsuccessfully in 0:01:44
[00:04:34] Build completed unsuccessfully in 0:01:44
[00:04:34] make: *** [tidy] Error 1
[00:04:34] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c54cc28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1af1dc7d:start=1530279959801177714,finish=1530279959807736196,duration=6558482
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c9e0db5
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:026c5a04
$ dmesg | grep -i kill
