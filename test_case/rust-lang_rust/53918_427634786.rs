plain
[00:53:06] ................................................................................................i... 2200/4563
[00:53:10] .................................................................................................... 2300/4563
[00:53:14] .................................................................................................... 2400/4563
[00:53:18] .................................................................................................... 2500/4563
[00:53:21] ........iiiiiiiii................................................................................... 2600/4563
[00:53:27] .................................................................................................... 2800/4563
[00:53:31] .................................................................................................... 2900/4563
[00:53:34] ............................i....................................................................... 3000/4563
[00:53:37] ........................................................................................i.i..ii..... 3100/4563
---
[01:31:53] travis_fold:end:stage0-linkchecker

[01:31:53] travis_time:end:stage0-linkchecker:start=1538899941792582952,finish=1538899944042187862,duration=2249604910

[01:33:50] std/vec/struct.Vec.html:1350: broken link - std/std/cmp/trait.PartialOrd.html
[01:33:50] std/vec/struct.Vec.html:1700: broken link - std/std/cmp/trait.PartialOrd.html
[01:34:45] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:34:45] 
[01:34:45] 
[01:34:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:34:45] expected success, got: exit code: 101
[01:34:45] expected success, got: exit code: 101
[01:34:45] 
[01:34:45] 
[01:34:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:45] Build completed unsuccessfully in 0:46:26
[01:34:45] Makefile:58: recipe for target 'check' failed
[01:34:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2bf6801e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:183b6354:start=1538900124460003233,finish=1538900124591751604,duration=131748371
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d48c286
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a02c396
$ dmesg | grep -i kill
