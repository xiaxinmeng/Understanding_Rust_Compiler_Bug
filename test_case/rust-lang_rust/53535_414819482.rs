plain
travis_time:start:tidy
tidy check
[00:04:44] * 555 error codes
[00:04:44] * highest error code: E0712
[00:04:44] tidy error: libsyntax/feature_gate.rs:225: no tracking issue for feature const_transmute
[00:04:45] Expected a gate test for the feature 'const_transmute'.
[00:04:45] Hint: create a failing test file named 'feature-gate-const_transmute.rs'
[00:04:45]       in the 'ui' test suite, with its failures due to
[00:04:45]       missing usage of #![feature(const_transmute)].
[00:04:45] Hint: If you already have such a test and don't want to rename it,
[00:04:45]       you can also add a // gate-test-const_transmute line to the test file.
[00:04:45] tidy error: Found 1 features without a gate test.
[00:04:45] tidy error: libsyntax/feature_gate.rs:225: no tracking issue for feature const_transmute
[00:04:45] some tidy checks failed
[00:04:45] 
[00:04:45] 
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:45] 
[00:04:45] 
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:45] Build completed unsuccessfully in 0:00:53
[00:04:45] Build completed unsuccessfully in 0:00:53
[00:04:45] Makefile:79: recipe for target 'tidy' failed
[00:04:45] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0261126c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:044b36bf:start=1534884540033777515,finish=1534884540039846810,duration=6069295
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0186be64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:211d1fac
travis_time:start:211d1fac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22bb6c60
$ dmesg | grep -i kill
