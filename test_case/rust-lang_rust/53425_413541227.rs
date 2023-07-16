plain

[00:05:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:11] tidy error: /checkout/src/test/ui/consts/const-eval/issue-53401.rs: missing trailing newline
[00:05:13] some tidy checks failed
[00:05:13] 
[00:05:13] 
[00:05:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:13] 
[00:05:13] 
[00:05:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:13] Build completed unsuccessfully in 0:00:53
[00:05:13] Build completed unsuccessfully in 0:00:53
[00:05:13] Makefile:79: recipe for target 'tidy' failed
[00:05:13] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33fba5ae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:000bcaa4:start=1534424673809213276,finish=1534424673816794007,duration=7580731
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0253b5e4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14299bf4
travis_time:start:14299bf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08890759
$ dmesg | grep -i kill
