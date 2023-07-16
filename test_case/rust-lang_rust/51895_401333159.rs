plain

[00:04:37] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:38] tidy error: /checkout/src/librustc/traits/object_safety.rs:390: line longer than 100 chars
[00:04:39] some tidy checks failed
[00:04:39] 
[00:04:39] 
[00:04:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:39] 
[00:04:39] 
[00:04:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:39] Build completed unsuccessfully in 0:01:44
[00:04:39] Build completed unsuccessfully in 0:01:44
[00:04:39] Makefile:79: recipe for target 'tidy' failed
[00:04:39] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a26d257
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:195f5bab:start=1530273075666930209,finish=1530273075674833508,duration=7903299
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04f5a092
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bc75870
$ dmesg | grep -i kill
