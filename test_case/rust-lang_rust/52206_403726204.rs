plain

[00:03:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:59] tidy error: /checkout/src/libcore/tests/slice.rs:389: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libcore/tests/slice.rs:416: TODO is deprecated; use FIXME
[00:04:01] some tidy checks failed
[00:04:01] 
[00:04:01] 
[00:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:01] 
[00:04:01] 
[00:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:01] Build completed unsuccessfully in 0:00:50
[00:04:01] Build completed unsuccessfully in 0:00:50
[00:04:01] make: *** [tidy] Error 1
[00:04:01] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1216931d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:055475b8:start=1531206865919441469,finish=1531206865925966084,duration=6524615
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d7bdf28
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1898c4c7
$ dmesg | grep -i kill
