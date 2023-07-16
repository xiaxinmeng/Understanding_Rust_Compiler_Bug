plain

[00:05:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:185: line longer than 100 chars
[00:05:01] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:240: line longer than 100 chars
[00:05:02] some tidy checks failed
[00:05:02] 
[00:05:02] 
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:02] 
[00:05:02] 
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:01:33
[00:05:02] Build completed unsuccessfully in 0:01:33
[00:05:02] make: *** [tidy] Error 1
[00:05:02] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:005a68ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fa43bf1:start=1530665234959664446,finish=1530665234967328121,duration=7663675
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:223ba2c2
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01773228
$ dmesg | grep -i kill
