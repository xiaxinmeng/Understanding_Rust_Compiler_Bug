plain

[00:05:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:13] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:411: trailing whitespace
[00:05:13] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:415: trailing whitespace
[00:05:13] tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:416: trailing whitespace
[00:05:14] some tidy checks failed
[00:05:14] 
[00:05:14] 
[00:05:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:14] 
[00:05:14] 
[00:05:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:14] Build completed unsuccessfully in 0:00:55
[00:05:14] Build completed unsuccessfully in 0:00:55
[00:05:14] Makefile:79: recipe for target 'tidy' failed
[00:05:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24bbee31
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0bb76fd0:start=1535651708407390286,finish=1535651708413681767,duration=6291481
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dd29670
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e82c08
travis_time:start:01e82c08
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c067a0e
$ dmesg | grep -i kill
