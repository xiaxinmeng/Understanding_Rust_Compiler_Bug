plain
[01:44:00] 
[01:44:00] testing https://github.com/servo/servo
[01:44:00] Initialized empty Git repository in /checkout/obj/build/ct/servo/.git/
[01:44:00] fatal: Could not parse object '17e97b9320fdb7cdb33bbc5f4d0fde0653bbf2e4'.
[01:44:56] fatal: unable to access 'https://github.com/servo/servo/': Could not resolve host: github.com
[01:44:56] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:44:56] 
[01:44:56] 
[01:44:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:44:56] expected success, got: exit code: 101
[01:44:56] expected success, got: exit code: 101
[01:44:56] 
[01:44:56] 
[01:44:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:44:56] Build completed unsuccessfully in 0:41:17
[01:44:56] Makefile:60: recipe for target 'check-aux' failed
[01:44:56] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:167a5f28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17dc8f95:start=1530859597576502812,finish=1530859597584451177,duration=7948365
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f005aa4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0829cc92
$ dmesg | grep -i kill
