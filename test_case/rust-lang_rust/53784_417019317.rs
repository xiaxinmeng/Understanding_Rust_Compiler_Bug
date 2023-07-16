plain
[00:47:03] ....................................................................................................
[00:47:06] ....................................................................................................
[00:47:08] ..........i.........................................................................................
[00:47:11] ....................................................................................................
[00:47:14] ...........................................................iiiiiiiii................................
[00:47:19] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:25] .......................................i............................................................
[00:47:28] .........................................................................................i.i..ii....
---
[01:13:12] travis_fold:end:stage0-linkchecker

[01:13:12] travis_time:end:stage0-linkchecker:start=1535560532150584350,finish=1535560534543569111,duration=2392984761

[01:13:21] std/slice/fn.from_raw_parts.html:13: broken link - primitive.pointer.html
[01:13:23] core/slice/fn.from_raw_parts.html:13: broken link - primitive.pointer.html
[01:13:27] alloc/slice/fn.from_raw_parts.html:13: broken link - primitive.pointer.html
[01:13:27] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:13:27] 
[01:13:27] 
[01:13:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:13:27] expected success, got: exit code: 101
[01:13:27] expected success, got: exit code: 101
[01:13:27] 
[01:13:27] 
[01:13:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:27] Build completed unsuccessfully in 0:30:13
[01:13:27] make: *** [check] Error 1
[01:13:27] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02373339
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06da3084:start=1535560551490692305,finish=1535560551501613226,duration=10920921
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22ac34c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23616c00
travis_time:start:23616c00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:170ac800
$ dmesg | grep -i kill
