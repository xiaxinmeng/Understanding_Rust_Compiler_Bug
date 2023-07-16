plain

[00:04:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:50] tidy error: /checkout/src/libcore/slice/mod.rs:3883: line longer than 100 chars
[00:04:50] tidy error: /checkout/src/libcore/slice/mod.rs:3902: line longer than 100 chars
[00:04:52] some tidy checks failed
[00:04:52] 
[00:04:52] 
[00:04:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:52] 
[00:04:52] 
[00:04:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:52] Build completed unsuccessfully in 0:00:54
[00:04:52] Build completed unsuccessfully in 0:00:54
[00:04:52] Makefile:79: recipe for target 'tidy' failed
[00:04:52] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ab94ed4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03924ab0:start=1536047819682215665,finish=1536047819691596104,duration=9380439
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1277be8a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07a8d890
travis_time:start:07a8d890
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:38ab21eb
$ dmesg | grep -i kill
