plain
[00:47:12] ...........................................................................i........................
[00:47:16] ....................................................................................................
[00:47:22] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:33] ........i.................iiiiiiiii...................................................
[00:47:33] 
[00:47:33] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:48:24] ...........................................................................i........................
[00:48:28] ....................................................................................................
[00:48:33] ....................................................................................................
[00:48:39] ....................................................................................................
[00:48:43] ........i.................iiiiiiiii...................................................
[00:48:43] 
[00:48:43]  finished in 70.545
[00:48:43] travis_fold:end:test_ui_nll

---
[01:27:12] travis_fold:end:stage0-linkchecker

[01:27:12] travis_time:end:stage0-linkchecker:start=1528111839626278601,finish=1528111842602995470,duration=2976716869

[01:27:20] std/vec/struct.Vec.html:1005: broken link - std/std/slice/struct.ExactChunks.html
[01:27:20] std/vec/struct.Vec.html:1024: broken link - std/std/slice/struct.ExactChunksMut.html
[01:27:21] std/slice/index.html:193: broken link fragment `#method.into_remainder` pointing to `std/slice/index.html`
[01:27:25] core/slice/index.html:147: broken link fragment `#method.into_remainder` pointing to `core/slice/index.html`
[01:27:30] alloc/slice/index.html:193: broken link fragment `#method.into_remainder` pointing to `alloc/slice/index.html`
[01:27:31] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:27:31] 
[01:27:31] 
[01:27:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:27:31] expected success, got: exit code: 101
[01:27:31] expected success, got: exit code: 101
[01:27:31] 
[01:27:31] 
[01:27:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:31] Build completed unsuccessfully in 0:42:38
[01:27:31] make: *** [check] Error 1
[01:27:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e2e2a60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
