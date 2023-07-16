plain

[00:03:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:56] tidy error: duplicate error code: 711
[00:03:56] tidy error: /checkout/src/librustc_mir/diagnostics.rs:2254: E0711: r##"
[00:03:56] tidy error: /checkout/src/librustc/diagnostics.rs:2165:     E0711, // a feature has been declared with conflicting stability attributes
[00:03:57] some tidy checks failed
[00:03:57] 
[00:03:57] 
[00:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:57] 
[00:03:57] 
[00:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:57] Build completed unsuccessfully in 0:00:49
[00:03:57] Build completed unsuccessfully in 0:00:49
[00:03:57] Makefile:79: recipe for target 'tidy' failed
[00:03:57] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d610a8f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1540cfa8:start=1533589225287236463,finish=1533589225294251934,duration=7015471
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a16210
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00612348
travis_time:start:00612348
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d81be40
$ dmesg | grep -i kill
