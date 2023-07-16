plain

[00:04:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:48] tidy error: /checkout/src/test/run-make-fulldeps/issue-51671/app.rs: incorrect license
[00:04:49] some tidy checks failed
[00:04:49] 
[00:04:49] 
[00:04:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:49] 
[00:04:49] 
[00:04:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:49] Build completed unsuccessfully in 0:01:44
[00:04:49] Build completed unsuccessfully in 0:01:44
[00:04:49] Makefile:79: recipe for target 'tidy' failed
[00:04:49] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1eed1000
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24f472dd:start=1529604655958727064,finish=1529604655966101255,duration=7374191
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16ee8967
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07260aeb
$ dmesg | grep -i kill
