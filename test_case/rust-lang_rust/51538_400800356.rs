plain

[00:05:32] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:32] tidy error: /checkout/src/librustc/traits/query/type_op/outlives.rs:37: line longer than 100 chars
[00:05:32] tidy error: /checkout/src/librustc/traits/query/type_op/eq.rs:30: line longer than 100 chars
[00:05:32] tidy error: /checkout/src/librustc/traits/query/type_op/prove_predicate.rs:33: line longer than 100 chars
[00:05:34] some tidy checks failed
[00:05:34] 
[00:05:34] 
[00:05:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:34] 
[00:05:34] 
[00:05:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:34] Build completed unsuccessfully in 0:02:02
[00:05:34] Build completed unsuccessfully in 0:02:02
[00:05:34] make: *** [tidy] Error 1
[00:05:34] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0486b058
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0287915c:start=1530127503233270638,finish=1530127503239169133,duration=5898495
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0346bb20
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03a1e74c
$ dmesg | grep -i kill
