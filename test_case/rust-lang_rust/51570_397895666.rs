plain

[00:04:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:50] tidy error: /checkout/src/librustc_metadata/encoder.rs:884: line longer than 100 chars
[00:04:50] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs: missing trailing newline
[00:04:51] some tidy checks failed
[00:04:51] 
[00:04:51] 
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:51] 
[00:04:51] 
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:51] Build completed unsuccessfully in 0:01:54
[00:04:51] Build completed unsuccessfully in 0:01:54
[00:04:51] make: *** [tidy] Error 1
[00:04:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04dec794
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:002e7aa8:start=1529258009527399579,finish=1529258009536769978,duration=9370399
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:246c6d57
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2fe1f5da
$ dmesg | grep -i kill
