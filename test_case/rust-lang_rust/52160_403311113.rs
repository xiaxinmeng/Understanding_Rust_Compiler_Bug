plain

[00:03:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:42] tidy error: /checkout/src/test/ui/include-macros/mismatched-types.rs: incorrect license
[00:03:44] some tidy checks failed
[00:03:44] 
[00:03:44] 
[00:03:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:44] 
[00:03:44] 
[00:03:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:44] Build completed unsuccessfully in 0:00:49
[00:03:44] Build completed unsuccessfully in 0:00:49
[00:03:44] Makefile:79: recipe for target 'tidy' failed
[00:03:44] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0405c8a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:07984ae2:start=1531077898236735512,finish=1531077898242914382,duration=6178870
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:196db2be
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:398f19d2
$ dmesg | grep -i kill
