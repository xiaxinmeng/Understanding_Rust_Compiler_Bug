plain

[00:04:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:04] tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:17: line longer than 100 chars
[00:04:04] tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:962: tab character
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:00:50
[00:04:06] Build completed unsuccessfully in 0:00:50
[00:04:06] Makefile:79: recipe for target 'tidy' failed
[00:04:06] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0164fd2a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:11f52fdd:start=1538146472103555952,finish=1538146472108290761,duration=4734809
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fe01ef4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dfa87e8
travis_time:start:0dfa87e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0862a303
$ dmesg | grep -i kill
