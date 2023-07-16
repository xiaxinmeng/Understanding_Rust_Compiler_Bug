plain

[00:05:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:11] tidy error: /checkout/src/librustc_passes/rvalue_promotion.rs:282: line longer than 100 chars
[00:05:13] some tidy checks failed
[00:05:13] 
[00:05:13] 
[00:05:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:13] 
[00:05:13] 
[00:05:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:13] Build completed unsuccessfully in 0:01:52
[00:05:13] Build completed unsuccessfully in 0:01:52
[00:05:13] Makefile:79: recipe for target 'tidy' failed
[00:05:13] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b05a4ec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2ac0b7c4:start=1530638890810598720,finish=1530638890817473716,duration=6874996
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:066e1926
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c5b451c
$ dmesg | grep -i kill
