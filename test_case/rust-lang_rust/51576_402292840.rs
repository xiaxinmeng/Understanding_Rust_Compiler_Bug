plain

[00:04:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:23] tidy error: /checkout/src/librustc_target/abi/mod.rs: missing trailing newline
[00:04:25] some tidy checks failed
[00:04:25] 
[00:04:25] 
[00:04:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:25] 
[00:04:25] 
[00:04:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:25] Build completed unsuccessfully in 0:01:32
[00:04:25] Build completed unsuccessfully in 0:01:32
[00:04:25] Makefile:79: recipe for target 'tidy' failed
[00:04:25] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b5e0b8a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05b5f225:start=1530651753822680019,finish=1530651753830666525,duration=7986506
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a099a80
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0891f0a4
$ dmesg | grep -i kill
