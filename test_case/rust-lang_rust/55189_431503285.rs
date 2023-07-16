plain
[00:49:15] .................................................................................................... 2200/4635
[00:49:19] ....................................i............................................................... 2300/4635
[00:49:23] .................................................................................................... 2400/4635
[00:49:27] .................................................................................................... 2500/4635
[00:49:30] ..................................................iiiiiiiii......................................... 2600/4635
[00:49:37] .................................................................................................... 2800/4635
[00:49:40] .................................................................................................... 2900/4635
[00:49:43] ................................................................................i................... 3000/4635
[00:49:46] .................................................................................................... 3100/4635
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:28] 
[01:02:28] running 111 tests
[01:02:31] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:02:32] ..iiii.....
[01:02:32] 
[01:02:32]  finished in 3.499
[01:02:32] travis_fold:end:test_codegen

---
[01:29:42] travis_fold:end:stage0-linkchecker

[01:29:42] travis_time:end:stage0-linkchecker:start=1539983922419828395,finish=1539983924581981106,duration=2162152711

[01:33:32] reference/crates-and-source-files.html:228: broken link - core/prelude.index.html
[01:33:32] reference/print.html:1740: broken link - core/prelude.index.html
[01:34:04] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:34:04] 
[01:34:04] 
[01:34:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:34:04] expected success, got: exit code: 101
[01:34:04] expected success, got: exit code: 101
[01:34:04] 
[01:34:04] 
[01:34:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:04] Build completed unsuccessfully in 0:49:28
[01:34:04] make: *** [check] Error 1
[01:34:04] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06510780
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0385f38d:start=1539984190583901957,finish=1539984190751386890,duration=167484933
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e21236a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006fb554
$ dmesg | grep -i kill
