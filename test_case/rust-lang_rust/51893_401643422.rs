plain

[00:04:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:36] tidy error: /checkout/src/liballoc/collections/btree/map.rs:217: XXX is deprecated; use FIXME
[00:04:37] some tidy checks failed
[00:04:37] 
[00:04:37] 
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:37] 
[00:04:37] 
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:37] Build completed unsuccessfully in 0:01:30
[00:04:37] Build completed unsuccessfully in 0:01:30
[00:04:37] make: *** [tidy] Error 1
[00:04:37] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2f3fbd6a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0a9b16a0:start=1530490126665110950,finish=1530490126671729842,duration=6618892
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f1121f0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11d491ba
$ dmesg | grep -i kill
