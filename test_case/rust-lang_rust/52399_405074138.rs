plain

[00:03:40] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:41] tidy error: /checkout/src/bootstrap/dist.rs:1913: line longer than 100 chars
[00:03:42] some tidy checks failed
[00:03:42] 
[00:03:42] 
[00:03:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:42] 
[00:03:42] 
[00:03:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:42] Build completed unsuccessfully in 0:00:49
[00:03:42] Build completed unsuccessfully in 0:00:49
[00:03:42] make: *** [tidy] Error 1
[00:03:42] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20ddbddd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:153d30aa:start=1531640622873858001,finish=1531640622884110857,duration=10252856
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02fe8a0e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b262e15
travis_time:start:1b262e15
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d953c00
$ dmesg | grep -i kill
