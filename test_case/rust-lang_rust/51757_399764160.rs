plain

[00:05:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:22] tidy error: /checkout/src/librustc_target/spec/x86_64_unknown_haiku.rs:20: trailing whitespace
[00:05:24] some tidy checks failed
[00:05:24] 
[00:05:24] 
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:24] 
[00:05:24] 
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:24] Build completed unsuccessfully in 0:01:57
[00:05:24] Build completed unsuccessfully in 0:01:57
[00:05:24] make: *** [tidy] Error 1
[00:05:24] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19e6de5d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0be45cc8:start=1529852981047450369,finish=1529852981054116015,duration=6665646
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0889cc8a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20bf4fee
$ dmesg | grep -i kill
