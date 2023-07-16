plain
[01:15:50] travis_fold:end:stage0-linkchecker

[01:15:50] travis_time:end:stage0-linkchecker:start=1532547690616675272,finish=1532547693142869704,duration=2526194432

[01:16:03] reference/destructors.html:217: broken link - std/mem/union.ManuallyDrop.html
[01:16:03] reference/print.html:6644: broken link - std/mem/union.ManuallyDrop.html
[01:16:05] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:16:05] 
[01:16:05] 
[01:16:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:16:05] expected success, got: exit code: 101
[01:16:05] expected success, got: exit code: 101
[01:16:05] 
[01:16:05] 
[01:16:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:05] Build completed unsuccessfully in 0:30:50
[01:16:05] Makefile:58: recipe for target 'check' failed
[01:16:05] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e0645c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3377eb80:start=1532547710293822346,finish=1532547710303072311,duration=9249965
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01cea5d2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:32dd27d6
travis_time:start:32dd27d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e595516
$ dmesg | grep -i kill
