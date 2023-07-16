plain
[00:54:48] ....................................................................................................
[00:54:51] ...............................................................i....................................
[00:54:53] ....................................................................................................
[00:54:57] ....................................................................................................
[00:54:59] ............iiiiiiiii...............................................................................
[00:55:05] ....................................................................................................
[00:55:08] ................................................................................................i...
[00:55:11] ....................................................................................................
[00:55:14] ........................................................i.i..ii.....................................
---
[01:26:06] travis_fold:end:stage0-linkchecker

[01:26:06] travis_time:end:stage0-linkchecker:start=1538080846403656929,finish=1538080848642144437,duration=2238487508

[01:28:13] std/sync/index.html:65: broken link - std/sync/std::sync::atomic::compiler_fence
[01:28:13] std/sync/index.html:79: broken link - std/sync/std::sync::atomic::fence
[01:28:13] std/sync/index.html:100: broken link - std/sync/std::sync::mpsc
[01:28:55] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:28:55] 
[01:28:55] 
[01:28:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:28:55] expected success, got: exit code: 101
[01:28:55] expected success, got: exit code: 101
[01:28:55] 
[01:28:55] 
[01:28:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:55] Build completed unsuccessfully in 0:42:57
[01:28:55] Makefile:58: recipe for target 'check' failed
[01:28:55] make: *** [check] Error 1
37756 ./src/tools/lldb/www
37080 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
37064 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36376 ./.git/modules/src/libcompiler_builtins
---
travis_time:end:03b35bfc:start=1538081019791091137,finish=1538081019797722661,duration=6631524
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:198b59ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00a6955a
travis_time:start:00a6955a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14a625bf
$ dmesg | grep -i kill
