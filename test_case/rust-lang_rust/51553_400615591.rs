plain

[00:05:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:04] tidy error: /checkout/src/libstd/sys/redox/ext/net.rs: incorrect license
[00:05:04] tidy error: /checkout/src/libstd/sys/unix/ext/unixsocket.rs:68: line longer than 100 chars
[00:05:04] tidy error: /checkout/src/libstd/sys/unix/ext/net.rs:326: line longer than 100 chars
[00:05:04] tidy error: /checkout/src/libstd/sys/unix/ext/net.rs:348: line longer than 100 chars
[00:05:05] tidy error: /checkout/src/libstd/sys_common/unixsocket.rs:11: platform-specific cfg: cfg(any(all(unix, not(target_os = "emscripten")), target_os = "redox"))
[00:05:06] some tidy checks failed
[00:05:06] 
[00:05:06] 
[00:05:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:06] 
[00:05:06] 
[00:05:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:06] Build completed unsuccessfully in 0:01:44
[00:05:06] Build completed unsuccessfully in 0:01:44
[00:05:06] make: *** [tidy] Error 1
[00:05:06] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ad26b6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:058847c4:start=1530093251003287717,finish=1530093251010014644,duration=6726927
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ca4bef0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b677229
$ dmesg | grep -i kill
