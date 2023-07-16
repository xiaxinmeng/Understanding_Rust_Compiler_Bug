plain

[00:04:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:29] tidy error: /checkout/src/librustc_mir/transform/qualify_min_const_fn.rs:185: line longer than 100 chars
[00:04:30] Expected a gate test for the feature 'min_const_fn'.
[00:04:30] Hint: create a failing test file named 'feature-gate-min_const_fn.rs'
[00:04:30]       in the 'ui' test suite, with its failures due to
[00:04:30]       missing usage of #![feature(min_const_fn)].
[00:04:30] Hint: If you already have such a test and don't want to rename it,
[00:04:30]       you can also add a // gate-test-min_const_fn line to the test file.
[00:04:30] tidy error: Found 1 features without a gate test.
[00:04:31] some tidy checks failed
[00:04:31] 
[00:04:31] 
[00:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:31] 
[00:04:31] 
[00:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:31] Build completed unsuccessfully in 0:00:49
[00:04:31] Build completed unsuccessfully in 0:00:49
[00:04:31] Makefile:79: recipe for target 'tidy' failed
[00:04:31] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:31a34363
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:009a1692:start=1534947260149227612,finish=1534947260155652245,duration=6424633
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09b1d2ef
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:013245c3
travis_time:start:013245c3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a8550a1
$ dmesg | grep -i kill
