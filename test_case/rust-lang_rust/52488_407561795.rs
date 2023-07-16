plain

[00:04:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:09] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/relate_tys.rs:408: trailing whitespace
[00:04:09] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:413: line longer than 100 chars
[00:04:09] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/values.rs:247: line longer than 100 chars
[00:04:09] tidy error: /checkout/src/librustc_mir/transform/remove_noop_landing_pads.rs:44: line longer than 100 chars
[00:04:11] some tidy checks failed
[00:04:11] 
[00:04:11] 
[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:11] 
[00:04:11] 
[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:11] Build completed unsuccessfully in 0:00:55
[00:04:11] Build completed unsuccessfully in 0:00:55
[00:04:11] Makefile:79: recipe for target 'tidy' failed
[00:04:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f0fbc00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15e2d8b6:start=1532467985463378451,finish=1532467985470268677,duration=6890226
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01fec608
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f6ceb30
travis_time:start:1f6ceb30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f9d18c
$ dmesg | grep -i kill
