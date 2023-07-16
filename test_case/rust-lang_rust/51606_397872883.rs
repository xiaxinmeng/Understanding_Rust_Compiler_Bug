plain

[00:04:58] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:58] tidy error: /checkout/src/libcore/slice/mod.rs:3963: line longer than 100 chars
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:01:48
[00:05:00] Build completed unsuccessfully in 0:01:48
[00:05:00] make: *** [tidy] Error 1
[00:05:00] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f3df89c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0f682dde:start=1529235001557376032,finish=1529235001566097635,duration=8721603
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e62ed2
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:101bc604
$ dmesg | grep -i kill
