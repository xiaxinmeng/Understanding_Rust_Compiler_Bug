plain

[00:04:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:46] tidy error: /checkout/src/libcore/cell.rs:1235: line longer than 100 chars
[00:04:48] some tidy checks failed
[00:04:48] 
[00:04:48] 
[00:04:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:48] 
[00:04:48] 
[00:04:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:48] Build completed unsuccessfully in 0:01:44
[00:04:48] Build completed unsuccessfully in 0:01:44
[00:04:48] Makefile:79: recipe for target 'tidy' failed
[00:04:48] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f9f2576
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:064dc514:start=1528650682346797564,finish=1528650682352809085,duration=6011521
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008be7d6
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b4cffe0
$ dmesg | grep -i kill
