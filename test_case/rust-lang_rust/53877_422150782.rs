plain
[00:54:34] ....................................................................................................
[00:54:37] .....................................................i..............................................
[00:54:39] ....................................................................................................
[00:54:42] ....................................................................................................
[00:54:45] .iiiiiiiii..........................................................................................
[00:54:51] ....................................................................................................
[00:54:54] ..................................................................................i.................
[00:54:57] ....................................................................................................
[00:55:00] ....................................i.i..ii.........................................................
---
[01:22:30] travis_fold:end:stage0-linkchecker

[01:22:30] travis_time:end:stage0-linkchecker:start=1537214327128195563,finish=1537214329456200953,duration=2328005390

[01:25:01] core/pin/index.html:14: broken link - core/boxed/struct.Box.html
[01:25:33] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:25:33] 
[01:25:33] 
[01:25:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:25:33] expected success, got: exit code: 101
[01:25:33] expected success, got: exit code: 101
[01:25:33] 
[01:25:33] 
[01:25:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:33] Build completed unsuccessfully in 0:39:32
[01:25:33] Makefile:58: recipe for target 'check' failed
[01:25:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29a02034
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:19278238:start=1537214515003545589,finish=1537214515010265896,duration=6720307
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16aa29b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then
