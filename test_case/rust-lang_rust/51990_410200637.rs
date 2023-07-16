plain
travis_time:start:tidy
tidy check
[00:03:36] * 550 error codes
[00:03:36] * highest error code: E0710
[00:03:36] tidy error: Found 1 features without a gate test.
[00:03:36] Expected a gate test for the feature 'const_compare_raw_pointers'.
[00:03:36] Hint: create a failing test file named 'feature-gate-const_compare_raw_pointers.rs'
[00:03:36]       in the 'ui' test suite, with its failures due to
[00:03:36]       missing usage of #![feature(const_compare_raw_pointers)].
[00:03:36] Hint: If you already have such a test and don't want to rename it,
[00:03:36]       you can also add a // gate-test-const_compare_raw_pointers line to the test file.
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:47
[00:03:37] Build completed unsuccessfully in 0:00:47
[00:03:37] Makefile:79: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f5daf4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2bc652c4:start=1533288423813682659,finish=1533288423819625206,duration=5942547
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05e743e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18eccf54
travis_time:start:18eccf54
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:014912a4
$ dmesg | grep -i kill
