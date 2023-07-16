plain
[00:58:45] ....................................................................................................
[00:58:48] .....................................................i..............................................
[00:58:51] ....................................................................................................
[00:58:54] ....................................................................................................
[00:58:56] ..iiiiiiiii.........................................................................................
[00:59:02] ....................................................................................................
[00:59:05] ......................................................................................i.............
[00:59:08] ....................................................................................................
[00:59:11] .........................................i.i..ii....................................................
---
[01:31:17] travis_fold:end:stage0-linkchecker

[01:31:17] travis_time:end:stage0-linkchecker:start=1537846164596359862,finish=1537846167158960093,duration=2562600231

[01:33:21] alloc/boxed/index.html:11: broken link - alloc/ops/trait.Deref.html
[01:33:48] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:33:48] 
[01:33:48] 
[01:33:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:33:48] expected success, got: exit code: 101
[01:33:48] expected success, got: exit code: 101
[01:33:48] 
[01:33:48] 
[01:33:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:48] Build completed unsuccessfully in 0:43:59
[01:33:48] make: *** [check] Error 1
[01:33:48] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25db7bd8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
