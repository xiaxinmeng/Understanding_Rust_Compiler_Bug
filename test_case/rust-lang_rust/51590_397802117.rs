plain

[00:04:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:31] tidy error: /checkout/src/librustc_codegen_utils/llvm_target_features.rs: incorrect license
[00:04:33] some tidy checks failed
[00:04:33] 
[00:04:33] 
[00:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:33] 
[00:04:33] 
[00:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:33] Build completed unsuccessfully in 0:01:44
[00:04:33] Build completed unsuccessfully in 0:01:44
[00:04:33] Makefile:79: recipe for target 'tidy' failed
[00:04:33] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2763baef
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e339b4c:start=1529143037664031161,finish=1529143037672207796,duration=8176635
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:098e0509
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:140f6770
$ dmesg | grep -i kill
