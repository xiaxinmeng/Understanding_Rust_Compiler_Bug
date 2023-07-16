plain

[00:03:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:47] tidy error: /checkout/src/librustc_codegen_llvm/mir/analyze.rs: incorrect license
[00:03:47] tidy error: /checkout/src/librustc_codegen_llvm/mir/analyze.rs: missing trailing newline
[00:03:49] some tidy checks failed
[00:03:49] 
[00:03:49] 
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:49] 
[00:03:49] 
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:49] Build completed unsuccessfully in 0:00:52
[00:03:49] Build completed unsuccessfully in 0:00:52
[00:03:49] Makefile:79: recipe for target 'tidy' failed
[00:03:49] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002933f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09b585b4:start=1531223009751336422,finish=1531223009761388773,duration=10052351
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0855aaf7
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e68dcb
$ dmesg | grep -i kill
