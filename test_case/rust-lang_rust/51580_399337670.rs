plain

[00:04:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:46] tidy error: duplicate error code: 704
[00:04:46] tidy error: /checkout/src/libsyntax/diagnostic_list.rs:401:     E0704, // incorrect visibility restriction
[00:04:46] tidy error: /checkout/src/librustc/diagnostics.rs:2138:     E0704, // multiple elided lifetimes used in arguments of `async fn`
[00:04:46] tidy error: duplicate error code: 703
[00:04:46] tidy error: /checkout/src/libsyntax/diagnostic_list.rs:400:     E0703, // invalid ABI
[00:04:46] tidy error: /checkout/src/librustc/diagnostics.rs:2137:     E0703, // multiple different lifetimes used in arguments of `async fn`
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:50
[00:04:47] Build completed unsuccessfully in 0:01:50
[00:04:47] make: *** [tidy] Error 1
[00:04:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2fa4ea41
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15841af6:start=1529649158410750532,finish=1529649158417220070,duration=6469538
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b07204
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:185d05be
$ dmesg | grep -i kill
