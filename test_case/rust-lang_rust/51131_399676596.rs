plain
travis_time:start:tidy
tidy check
[00:04:51] * 553 error codes
[00:04:51] * highest error code: E0906
[00:04:51] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:04:51] 
[00:04:51] 
[00:04:51] 
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:51] 
[00:04:51] 
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:51] Build completed unsuccessfully in 0:01:45
[00:04:51] Build completed unsuccessfully in 0:01:45
[00:04:51] make: *** [tidy] Error 1
[00:04:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21fc47b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1abfdbd8:start=1529758346121385062,finish=1529758346128758612,duration=7373550
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b2f8e40
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b041a05
$ dmesg | grep -i kill
