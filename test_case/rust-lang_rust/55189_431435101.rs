plain
[00:54:20] .................................................................................................... 2200/4635
[00:54:24] ....................................i............................................................... 2300/4635
[00:54:28] .................................................................................................... 2400/4635
[00:54:32] .................................................................................................... 2500/4635
[00:54:35] ..................................................iiiiiiiii......................................... 2600/4635
[00:54:42] .................................................................................................... 2800/4635
[00:54:46] .................................................................................................... 2900/4635
[00:54:49] ................................................................................i................... 3000/4635
[00:54:52] .................................................................................................... 3100/4635
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:39] 
[01:08:39] running 111 tests
[01:08:42] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:08:42] ..iiii.....
[01:08:42] 
[01:08:42]  finished in 3.690
[01:08:42] travis_fold:end:test_codegen

---
[01:37:34] travis_fold:end:stage0-linkchecker

[01:37:34] travis_time:end:stage0-linkchecker:start=1539969345916937258,finish=1539969348329870736,duration=2412933478

[01:39:47] reference/crates-and-source-files.html:228: broken link - core/preludce.index.html
[01:39:47] reference/print.html:1740: broken link - core/preludce.index.html
[01:40:06] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:40:06] 
[01:40:06] 
[01:40:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:40:06] expected success, got: exit code: 101
[01:40:06] expected success, got: exit code: 101
[01:40:06] 
[01:40:06] 
[01:40:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:06] Build completed unsuccessfully in 0:50:42
[01:40:06] Makefile:58: recipe for target 'check' failed
[01:40:06] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09390cdb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
