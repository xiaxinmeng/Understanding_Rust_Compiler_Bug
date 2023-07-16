plain

[00:04:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:56] tidy error: /checkout/src/libcore/marker.rs:141: line longer than 100 chars
[00:04:58] some tidy checks failed
[00:04:58] 
[00:04:58] 
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:58] 
[00:04:58] 
[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:58] Build completed unsuccessfully in 0:00:49
[00:04:58] Build completed unsuccessfully in 0:00:49
[00:04:58] Makefile:79: recipe for target 'tidy' failed
[00:04:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b199510
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0586e7c8:start=1535502747745384388,finish=1535502747756001615,duration=10617227
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10ee3d5a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04314c03
travis_time:start:04314c03
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06438641
$ dmesg | grep -i kill
