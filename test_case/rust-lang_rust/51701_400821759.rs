plain

[00:05:07] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:07] tidy error: /checkout/src/libcore/slice/mod.rs:1544: trailing whitespace
[00:05:07] tidy error: /checkout/src/libcore/slice/mod.rs:1613: trailing whitespace
[00:05:09] some tidy checks failed
[00:05:09] 
[00:05:09] 
[00:05:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:09] 
[00:05:09] 
[00:05:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:09] Build completed unsuccessfully in 0:01:53
[00:05:09] Build completed unsuccessfully in 0:01:53
[00:05:09] Makefile:79: recipe for target 'tidy' failed
[00:05:09] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:044bb639
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0bb6a241:start=1530131408637635592,finish=1530131408644054217,duration=6418625
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bec1928
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3ce9b0dc
$ dmesg | grep -i kill
