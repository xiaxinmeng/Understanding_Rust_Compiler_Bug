plain
[00:54:26] ....i....................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:31] ...........
[00:55:04] ....................................................................................................
[00:55:34] ......................................................................ii............................
[00:56:26] .................................i....................................................i.ii......test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:57:11] ..............................................................................................iiiiii
[00:57:39] i...................................................................................................
[00:58:09] ....................................................................................................
[00:58:35] ....................................................................................................
---
[01:31:18] travis_fold:end:stage0-linkchecker

[01:31:18] travis_time:end:stage0-linkchecker:start=1524433456208041777,finish=1524433459099048407,duration=2891006630

[01:31:28] book/unsized-types.html:46: broken link fragment `#dynamically-sized-types--sized` pointing to `book/second-edition/ch19-04-advanced-types.html`
[01:31:36] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:31:36] 
[01:31:36] 
[01:31:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:31:36] expected success, got: exit code: 101
[01:31:36] expected success, got: exit code: 101
[01:31:36] 
[01:31:36] 
[01:31:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:36] Build completed unsuccessfully in 0:47:55
[01:31:36] Makefile:58: recipe for target 'check' failed
[01:31:36] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b687e4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
