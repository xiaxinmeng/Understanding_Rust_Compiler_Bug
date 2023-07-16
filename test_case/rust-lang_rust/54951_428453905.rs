plain

[00:04:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:14] tidy error: /checkout/src/libstd/sys/wasm/thread_local_atomics.rs:42: TODO is deprecated; use FIXME
[00:04:14] tidy error: /checkout/src/libstd/sys/wasm/thread.rs:98: TODO is deprecated; use FIXME
[00:04:15] some tidy checks failed
[00:04:15] 
[00:04:15] 
[00:04:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:15] 
[00:04:15] 
[00:04:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:15] Build completed unsuccessfully in 0:00:42
[00:04:15] Build completed unsuccessfully in 0:00:42
[00:04:15] make: *** [tidy] Error 1
[00:04:15] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2203cbf0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0072fa74:start=1539152573695837699,finish=1539152573699600477,duration=3762778
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c48a92
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:360961b8
travis_time:start:360961b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:32b53a88
$ dmesg | grep -i kill
