plain
[00:45:50] ..............................................................................i.....................
[00:45:55] ....................................................................................................
[00:46:01] ....................................................................................................
[00:46:08] ....................................................................................................
[00:46:13] ...........i.................iiiiiiiii...................................................
[00:46:13] 
[00:46:13] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:47:06] ..............................................................................i.....................
[00:47:11] ....................................................................................................
[00:47:16] ....................................................................................................
[00:47:22] ....................................................................................................
[00:47:27] ...........i.................iiiiiiiii...................................................
[00:47:27] 
[00:47:27]  finished in 74.142
[00:47:27] travis_fold:end:test_ui_nll

---
[01:26:55] travis_fold:end:stage0-linkchecker

[01:26:55] travis_time:end:stage0-linkchecker:start=1528206232535039354,finish=1528206235691055790,duration=3156016436

[01:27:04] std/cell/struct.Cell.html:484: broken link fragment `#method.sort_by_key` pointing to `std/cell/struct.Cell.html`
[01:27:04] std/cell/struct.Cell.html:551: broken link fragment `#method.make_ascii_uppercase` pointing to `std/cell/struct.Cell.html`
[01:27:04] std/cell/struct.Cell.html:556: broken link fragment `#method.make_ascii_lowercase` pointing to `std/cell/struct.Cell.html`
[01:27:13] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:27:13] 
[01:27:13] 
[01:27:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:27:13] expected success, got: exit code: 101
[01:27:13] expected success, got: exit code: 101
[01:27:13] 
[01:27:13] 
[01:27:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:13] Build completed unsuccessfully in 0:43:45
[01:27:13] make: *** [check] Error 1
[01:27:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09e5bbff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
