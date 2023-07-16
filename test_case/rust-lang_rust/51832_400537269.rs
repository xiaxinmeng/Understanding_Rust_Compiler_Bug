plain

[00:05:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:04] tidy error: /checkout/src/libstd/net/ip.rs:512: line longer than 100 chars
[00:05:04] tidy error: /checkout/src/libstd/net/ip.rs:1177: line longer than 100 chars
[00:05:05] some tidy checks failed
[00:05:05] 
[00:05:05] 
[00:05:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:05] 
[00:05:05] 
[00:05:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:05] Build completed unsuccessfully in 0:01:49
[00:05:05] Build completed unsuccessfully in 0:01:49
[00:05:05] make: *** [tidy] Error 1
[00:05:05] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02174ee0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:145bcdbb:start=1530072386266320854,finish=1530072386273396528,duration=7075674
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3a4bd6ee
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:037306b0
$ dmesg | grep -i kill
