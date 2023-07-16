plain

[00:06:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:24] tidy error: /checkout/src/test/ui/const-eval/promote_const_fn_call.rs: missing trailing newline
[00:06:26] some tidy checks failed
[00:06:26] 
[00:06:26] 
[00:06:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:26] 
[00:06:26] 
[00:06:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:26] Build completed unsuccessfully in 0:01:45
[00:06:26] Build completed unsuccessfully in 0:01:45
[00:06:26] make: *** [tidy] Error 1
[00:06:26] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:105ffd62
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:16585d43:start=1529950994260730747,finish=1529950994266916394,duration=6185647
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08d53542
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e96c42
$ dmesg | grep -i kill
