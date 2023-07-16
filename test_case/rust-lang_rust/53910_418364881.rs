plain
[00:48:49] ....................................................................................................
[00:48:52] ....................................................................................................
[00:48:55] ...........................i........................................................................
[00:48:58] ....................................................................................................
[00:49:01] ............................................................................iiiiiiiii...............
[00:49:06] ....................................................................................................
[00:49:10] ....................................................................................................
[00:49:13] .........................................................i..........................................
[00:49:16] ....................................................................................................
---
[01:17:13] travis_fold:end:stage0-linkchecker

[01:17:13] travis_time:end:stage0-linkchecker:start=1536067314162849865,finish=1536067316525866419,duration=2363016554

[01:17:18] std/os/raw/enum.c_void.html:4: broken link - std/std/primitive.pointer.html
[01:17:18] std/os/raw/enum.c_void.html:8: broken link - std/std/primitive.never.html
[01:17:18] std/os/raw/index.html:13: broken link - std/std/primitive.pointer.html
[01:17:29] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:17:29] 
[01:17:29] 
[01:17:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:17:29] expected success, got: exit code: 101
[01:17:29] expected success, got: exit code: 101
[01:17:29] 
[01:17:29] 
[01:17:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:29] Build completed unsuccessfully in 0:32:46
[01:17:29] make: *** [check] Error 1
[01:17:29] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b68c3cb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
