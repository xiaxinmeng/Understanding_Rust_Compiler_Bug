plain
[00:52:43] ....................................................................................................
[00:52:46] ..............................................................i.....................................
[00:52:48] ....................................................................................................
[00:52:51] ....................................................................................................
[00:52:54] ...........iiiiiiiii................................................................................
[00:52:59] ....................................................................................................
[00:53:03] ...............................................................................................i....
[00:53:05] ....................................................................................................
[00:53:08] .......................................................i.i..ii......................................
---
[01:22:31] travis_fold:end:stage0-linkchecker

[01:22:31] travis_time:end:stage0-linkchecker:start=1537979558722995332,finish=1537979560968852290,duration=2245856958

[01:24:40] std/keyword.let.html:29: broken link - std/keyword.mut.html
[01:25:40] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:25:40] 
[01:25:40] 
[01:25:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:25:40] expected success, got: exit code: 101
[01:25:40] expected success, got: exit code: 101
[01:25:40] 
[01:25:40] 
[01:25:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:40] Build completed unsuccessfully in 0:41:15
[01:25:40] make: *** [check] Error 1
[01:25:40] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b494808
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06095d2d:start=1537979752064282989,finish=1537979752071532258,duration=7249269
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c368f7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0819843c
travis_time:start:0819843c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1bf91419
$ dmesg | grep -i kill
