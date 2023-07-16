plain

[00:04:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:11] tidy error: /checkout/src/librustc_traits/dropck_outlives.rs:37: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc_traits/evaluate_obligation.rs:29: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/infer/mod.rs:515: line longer than 100 chars
[00:04:12] some tidy checks failed
[00:04:12] 
[00:04:12] 
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:12] 
[00:04:12] 
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:12] Build completed unsuccessfully in 0:00:46
[00:04:12] Build completed unsuccessfully in 0:00:46
[00:04:12] make: *** [tidy] Error 1
[00:04:12] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036ba66c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05b627e0:start=1538764766927338529,finish=1538764766932375446,duration=5036917
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1304fcf0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d1bd67c
travis_time:start:0d1bd67c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d28ea21
$ dmesg | grep -i kill
