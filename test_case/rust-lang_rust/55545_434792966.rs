plain
travis_time:end:10273063:start=1541003248988526487,finish=1541003250107221169,duration=1118694682
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:54:59] .................................................................................................... 100/4983
[00:55:02] .................................................................................................... 200/4983
[00:55:05] ...........................................................................................ii....... 300/4983
[00:55:08] .........................................................................................iii........ 400/4983
[00:55:11] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:55:19] .................................................................................................... 700/4983
[00:55:25] ..................................................................i...........i..................... 800/4983
[00:55:28] ....................................................................................iiiii........... 900/4983
[00:55:32] .................................................................................................... 1000/4983
---
[00:56:10] .................................................................................................... 2200/4983
[00:56:14] .................................................................................................... 2300/4983
[00:56:19] .................................................................................................... 2400/4983
[00:56:23] .................................................................................................... 2500/4983
[00:56:26] ...................................................................iiiiiiiii........................ 2600/4983
[00:56:34] ..................ii................................................................................ 2800/4983
[00:56:36] .................................................................................................... 2900/4983
[00:56:41] .................................................................................................... 3000/4983
[00:56:43] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:06] 
[01:11:06] running 115 tests
[01:11:09] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:11:09] .i....iiii.....
[01:11:09] 
[01:11:09]  finished in 3.950
[01:11:09] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:26] 
[01:11:26] running 118 tests
[01:11:51] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:11:55] ......iii.i.....ii
[01:11:55] 
[01:11:55]  finished in 29.373
[01:11:55] travis_fold:end:test_debuginfo

---
[01:42:50] travis_fold:end:stage0-linkchecker

[01:42:50] travis_time:end:stage0-linkchecker:start=1541009428260257864,finish=1541009430570420873,duration=2310163009

[01:45:22] reference/introduction.html:156: broken link - grammar.html
[01:45:22] reference/types.html:396: broken link - grammar.html
[01:45:23] reference/notation.html:150: broken link - grammar.html
[01:45:23] reference/index.html:155: broken link - grammar.html
[01:45:23] reference/print.html:156: broken link - grammar.html
[01:45:23] reference/print.html:267: broken link - grammar.html
[01:45:23] reference/print.html:7223: broken link - grammar.html
[01:45:54] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:45:54] 
[01:45:54] 
[01:45:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:45:54] expected success, got: exit code: 101
[01:45:54] expected success, got: exit code: 101
[01:45:54] 
[01:45:54] 
[01:45:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:54] Build completed unsuccessfully in 0:54:53
[01:45:54] make: *** [check] Error 1
[01:45:54] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ba9892e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ff31524:start=1541009617539885869,finish=1541009617545069777,duration=5183908
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2880063c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f54089a
travis_time:start:1f54089a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0168583e
$ dmesg | grep -i kill
