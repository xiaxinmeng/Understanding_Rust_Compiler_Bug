plain
[00:47:37] ................................................................................................i... 2200/4563
[00:47:41] .................................................................................................... 2300/4563
[00:47:44] .................................................................................................... 2400/4563
[00:47:48] .................................................................................................... 2500/4563
[00:47:51] ........iiiiiiiii................................................................................... 2600/4563
[00:47:57] .................................................................................................... 2800/4563
[00:48:01] .................................................................................................... 2900/4563
[00:48:03] ............................i....................................................................... 3000/4563
[00:48:06] ........................................................................................i.i..ii..... 3100/4563
---
[01:22:53] travis_fold:end:stage0-linkchecker

[01:22:53] travis_time:end:stage0-linkchecker:start=1538908626887235069,finish=1538908629074560159,duration=2187325090

[01:22:53] std/primitive.slice.html:633: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/cmp/trait.PartialOrd.html
[01:22:53] std/primitive.slice.html:983: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/cmp/trait.PartialOrd.html
[01:26:02] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:26:02] 
[01:26:02] 
[01:26:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:26:02] expected success, got: exit code: 101
[01:26:02] expected success, got: exit code: 101
[01:26:02] 
[01:26:02] 
[01:26:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:02] Build completed unsuccessfully in 0:43:05
[01:26:02] make: *** [check] Error 1
[01:26:02] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03952c38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
