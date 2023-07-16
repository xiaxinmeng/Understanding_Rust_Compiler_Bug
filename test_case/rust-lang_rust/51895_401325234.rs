plain

[00:04:43] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:44] tidy error: /checkout/src/librustc/traits/object_safety.rs:390: line longer than 100 chars
[00:04:45] some tidy checks failed
[00:04:45] 
[00:04:45] 
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:45] 
[00:04:45] 
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:45] Build completed unsuccessfully in 0:01:52
[00:04:45] Build completed unsuccessfully in 0:01:52
[00:04:45] Makefile:79: recipe for target 'tidy' failed
[00:04:45] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06572b28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c941bec:start=1530270530316800648,finish=1530270530324793085,duration=7992437
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cb24100
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0244fac9
$ dmesg | grep -i kill
