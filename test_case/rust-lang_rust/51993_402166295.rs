plain

[00:03:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:36] tidy error: /checkout/src/librustc_passes/rvalue_promotion.rs:282: line longer than 100 chars
[00:03:38] some tidy checks failed
[00:03:38] 
[00:03:38] 
[00:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:38] 
[00:03:38] 
[00:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:38] Build completed unsuccessfully in 0:01:11
[00:03:38] Build completed unsuccessfully in 0:01:11
[00:03:38] Makefile:79: recipe for target 'tidy' failed
[00:03:38] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06043098
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ac0d2da:start=1530625632582330355,finish=1530625632586746654,duration=4416299
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07f0bb91
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0126f770
$ dmesg | grep -i kill
