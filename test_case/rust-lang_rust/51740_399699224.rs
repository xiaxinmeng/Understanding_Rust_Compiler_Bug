plain

[00:05:02] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:03] tidy error: duplicate error code: 706
[00:05:03] tidy error: /checkout/src/librustc_passes/diagnostics.rs:313:     E0706, // `async fn` in trait
[00:05:03] tidy error: /checkout/src/librustc/diagnostics.rs:2137:     E0706, // multiple different lifetimes used in arguments of `async fn`
[00:05:04] some tidy checks failed
[00:05:04] 
[00:05:04] 
[00:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:04] 
[00:05:04] 
[00:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:04] Build completed unsuccessfully in 0:01:51
[00:05:04] Build completed unsuccessfully in 0:01:51
[00:05:04] make: *** [tidy] Error 1
[00:05:04] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:144b8b95
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e85094c:start=1529778115620205043,finish=1529778115627262887,duration=7057844
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:267d38c4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:236fb798
$ dmesg | grep -i kill
