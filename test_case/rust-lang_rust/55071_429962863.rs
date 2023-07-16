plain

[00:04:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:15] tidy error: /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs: missing trailing newline
[00:04:17] some tidy checks failed
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:44
[00:04:17] Build completed unsuccessfully in 0:00:44
[00:04:17] make: *** [tidy] Error 1
[00:04:17] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2860b0a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:245facce:start=1539627467614135797,finish=1539627467618265609,duration=4129812
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07d32011
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b6aec7
travis_time:start:03b6aec7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06ad31a2
$ dmesg | grep -i kill
