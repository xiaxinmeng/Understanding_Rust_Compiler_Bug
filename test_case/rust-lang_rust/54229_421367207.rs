plain

[00:04:39] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:40] tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:541: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:591: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:423: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:424: line longer than 100 chars
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:48
[00:04:41] Build completed unsuccessfully in 0:00:48
[00:04:41] make: *** [tidy] Error 1
[00:04:41] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bfae0cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06ec06ce:start=1536933379470402990,finish=1536933379474680766,duration=4277776
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:130c9838
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02dcef0e
travis_time:start:02dcef0e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03c6f89d
$ dmesg | grep -i kill
