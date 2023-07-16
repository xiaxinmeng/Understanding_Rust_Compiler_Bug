plain
[00:58:45] .................................................................................................... 2200/4576
[00:58:49] ...i................................................................................................ 2300/4576
[00:58:53] .................................................................................................... 2400/4576
[00:58:56] .................................................................................................... 2500/4576
[00:59:00] ................iiiiiiiii........................................................................... 2600/4576
[00:59:06] .................................................................................................... 2800/4576
[00:59:10] .................................................................................................... 2900/4576
[00:59:13] ....................................i............................................................... 3000/4576
[00:59:15] ................................................................................................i.i. 3100/4576
---
[01:34:16] travis_fold:end:stage0-linkchecker

[01:34:16] travis_time:end:stage0-linkchecker:start=1539119317398368434,finish=1539119319783784010,duration=2385415576

[01:36:26] std/arch/wasm32/index.html:3: broken link - core/arch/wasm32/index.html
3302240 ./obj
3086216 ./obj/build
2450528 ./obj/build/x86_64-unknown-linux-gnu
1069576 ./src
---
travis_time:end:078cb0d9:start=1539119514198033178,finish=1539119514202838573,duration=4805395
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01791c0a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05ca71c0
travis_time:start:05ca71c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ad99eb7
$ dmesg | grep -i kill
