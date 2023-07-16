plain
[00:48:44] ....................................................................................................
[00:48:48] ....................................................................................................
[00:48:50] ......................i.............................................................................
[00:48:53] ....................................................................................................
[00:48:56] .......................................................................iiiiiiiii....................
[00:49:01] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:08] ....................................................i...............................................
[00:49:11] ....................................................................................................
---
[01:16:35] travis_fold:end:stage0-linkchecker

[01:16:35] travis_time:end:stage0-linkchecker:start=1536002897316875936,finish=1536002899729486704,duration=2412610768

[01:16:41] std/ffi/enum.c_void.html:4: broken link - primitive.pointer.html
[01:16:41] std/ffi/enum.c_void.html:8: broken link - primitive.never.html
[01:16:41] std/ffi/index.html:190: broken link - primitive.pointer.html
[01:16:45] core/ffi/enum.c_void.html:4: broken link - primitive.pointer.html
[01:16:45] core/ffi/enum.c_void.html:8: broken link - primitive.never.html
[01:16:45] core/ffi/index.html:8: broken link - primitive.pointer.html
[01:16:49] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:16:49] 
[01:16:49] 
[01:16:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:16:49] expected success, got: exit code: 101
[01:16:49] expected success, got: exit code: 101
[01:16:49] 
[01:16:49] 
[01:16:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:49] Build completed unsuccessfully in 0:32:08
[01:16:49] make: *** [check] Error 1
[01:16:49] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02e76944
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
