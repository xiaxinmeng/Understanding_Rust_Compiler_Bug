plain

[00:05:32] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:33] tidy error: /checkout/src/bootstrap/test.rs:242: trailing whitespace
[00:05:34] some tidy checks failed
[00:05:34] 
[00:05:34] 
[00:05:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:34] 
[00:05:34] 
[00:05:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:34] Build completed unsuccessfully in 0:00:54
[00:05:34] Build completed unsuccessfully in 0:00:54
[00:05:34] Makefile:79: recipe for target 'tidy' failed
[00:05:34] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:128eefce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0aeb74b0:start=1536488984423324351,finish=1536488984429906725,duration=6582374
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:018c6f44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ad16ce
travis_time:start:04ad16ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1eb2d2e3
$ dmesg | grep -i kill
