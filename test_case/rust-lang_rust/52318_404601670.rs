plain

[00:03:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:51] tidy error: /checkout/src/librustc_passes/rvalue_promotion.rs:640: line longer than 100 chars
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:00:52
[00:03:53] Build completed unsuccessfully in 0:00:52
[00:03:53] make: *** [tidy] Error 1
[00:03:53] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04ba20ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1660802a:start=1531418573223763121,finish=1531418573230183111,duration=6419990
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18878b08
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01051bfc
$ dmesg | grep -i kill
