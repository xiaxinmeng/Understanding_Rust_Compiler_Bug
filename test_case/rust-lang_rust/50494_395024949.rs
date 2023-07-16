plain
[00:42:33] i..............................................................................i....................
[00:42:38] ....................................................................................................
[00:42:44] ....................................................................................................
[00:42:50] ....................................................................................................
[00:42:54] ............i.................iiiiiiiii...................................................
[00:42:54] 
[00:42:54] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:43:44] i..............................................................................i....................
[00:43:48] ....................................................................................................
[00:43:53] ....................................................................................................
[00:43:59] ....................................................................................................
[00:44:03] ............i.................iiiiiiiii...................................................
[00:44:03] 
[00:44:03]  finished in 68.873
[00:44:03] travis_fold:end:test_ui_nll

---
[01:20:17] travis_fold:end:stage0-linkchecker

[01:20:17] travis_time:end:stage0-linkchecker:start=1528280150197190677,finish=1528280152970294814,duration=2773104137

[01:20:24] std/vec/struct.Vec.html:1652: broken link - std/std/primitive.slice.html
[01:20:24] std/vec/struct.Vec.html:1690: broken link - std/std/primitive.slice.html
[01:20:24] std/vec/struct.Vec.html:1695: broken link - std/std/primitive.slice.html
[01:20:25] std/cell/struct.Cell.html:484: broken link fragment `#method.sort_by_key` pointing to `std/cell/struct.Cell.html`
[01:20:25] std/cell/struct.Cell.html:551: broken link - std/std/primitive.slice.html
[01:20:25] std/cell/struct.Cell.html:556: broken link - std/std/primitive.slice.html
[01:20:33] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:20:33] 
[01:20:33] 
[01:20:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:20:33] expected success, got: exit code: 101
[01:20:33] expected success, got: exit code: 101
[01:20:33] 
[01:20:33] 
[01:20:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:33] Build completed unsuccessfully in 0:40:11
[01:20:33] Makefile:58: recipe for target 'check' failed
[01:20:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:211a1492
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
