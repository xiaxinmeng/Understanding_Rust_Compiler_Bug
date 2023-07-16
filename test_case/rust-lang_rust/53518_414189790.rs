plain
[00:55:55] ....................................................................................................
[00:55:59] ....................................................................................................
[00:56:01] .......i............................................................................................
[00:56:05] ....................................................................................................
[00:56:07] ........................................................iiiiiiiii...................................
[00:56:14] ....................................................................................................
[00:56:18] ....................................................................................................
[00:56:20] ....................................i...............................................................
[00:56:23] ......................................................................................i.i..ii.......
---
[01:27:26] travis_fold:end:stage0-linkchecker

[01:27:26] travis_time:end:stage0-linkchecker:start=1534736774109096078,finish=1534736776688149991,duration=2579053913

[01:27:31] std/primitive.u32.html:1147: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.char.html
[01:27:31] std/primitive.u32.html:1147: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u32.html
[01:27:34] std/primitive.char.html:883: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u8.html
[01:27:34] std/primitive.char.html:883: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.char.html
[01:27:40] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:27:40] 
[01:27:40] 
[01:27:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:27:40] expected success, got: exit code: 101
[01:27:40] expected success, got: exit code: 101
[01:27:40] 
[01:27:40] 
[01:27:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:40] Build completed unsuccessfully in 0:36:11
[01:27:40] Makefile:58: recipe for target 'check' failed
[01:27:40] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f1bae08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:041f67b2:start=1534736793974817838,finish=1534736794067225904,duration=92408066
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f9323f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03b71e6b
$ dmesg | grep -i kill
