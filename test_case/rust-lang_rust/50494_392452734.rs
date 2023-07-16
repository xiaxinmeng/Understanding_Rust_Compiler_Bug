plain
[00:48:49] ................................................................i...................................
[00:48:53] ....................................................................................................
[00:48:59] ....................................................................................................
[00:49:05] .............................................................................................i......
[00:49:08] ...........iiiiiiiii...................................................
[00:49:08] 
[00:49:08] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:49:56] ................................................................i...................................
[00:50:00] ....................................................................................................
[00:50:06] ....................................................................................................
[00:50:11] .............................................................................................i......
[00:50:14] ............iiiiiiiii..................................................
[00:50:14] 
[00:50:14]  finished in 66.166
[00:50:14] travis_fold:end:test_ui_nll

---
[01:28:06] travis_fold:end:stage0-linkchecker

[01:28:06] travis_time:end:stage0-linkchecker:start=1527494789754596949,finish=1527494792566771763,duration=2812174814

[01:28:14] std/cell/struct.Cell.html:484: broken link fragment `#method.sort_by_key` pointing to `std/cell/struct.Cell.html`
[01:28:14] std/cell/struct.Cell.html:551: broken link fragment `#method.make_ascii_uppercase` pointing to `std/cell/struct.Cell.html`
[01:28:14] std/cell/struct.Cell.html:556: broken link fragment `#method.make_ascii_lowercase` pointing to `std/cell/struct.Cell.html`
[01:28:24] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:28:24] 
[01:28:24] 
[01:28:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:28:24] expected success, got: exit code: 101
[01:28:24] expected success, got: exit code: 101
[01:28:24] 
[01:28:24] 
[01:28:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:24] Build completed unsuccessfully in 0:41:58
[01:28:24] make: *** [check] Error 1
[01:28:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c6cfbb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
