plain
[00:43:30] i..............................................................................i....................
[00:43:35] ....................................................................................................
[00:43:40] ....................................................................................................
[00:43:46] ....................................................................................................
[00:43:50] ............i.................iiiiiiiii...................................................
[00:43:50] 
[00:43:50] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:38] i..............................................................................i....................
[00:44:43] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:53] ....................................................................................................
[00:44:57] ............i.................iiiiiiiii...................................................
[00:44:57] 
[00:44:57]  finished in 67.009
[00:44:57] travis_fold:end:test_ui_nll

---
[01:21:50] travis_fold:end:stage0-linkchecker

[01:21:50] travis_time:end:stage0-linkchecker:start=1528268428979069727,finish=1528268431896076812,duration=2917007085

[01:21:50] std/primitive.slice.html:931: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.slice.html
[01:21:50] std/primitive.slice.html:979: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.slice.html
[01:21:50] std/primitive.slice.html:984: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.slice.html
[01:21:58] std/cell/struct.Cell.html:484: broken link fragment `#method.sort_by_key` pointing to `std/cell/struct.Cell.html`
[01:22:07] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:22:07] 
[01:22:07] 
[01:22:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:22:07] expected success, got: exit code: 101
[01:22:07] expected success, got: exit code: 101
[01:22:07] 
[01:22:07] 
[01:22:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:07] Build completed unsuccessfully in 0:40:50
[01:22:07] Makefile:58: recipe for target 'check' failed
[01:22:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a1b44af
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
