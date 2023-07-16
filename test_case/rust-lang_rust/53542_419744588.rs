plain
travis_time:start:tidy
tidy check
[00:04:40] * 555 error codes
[00:04:40] * highest error code: E0712
[00:04:40] Expected a gate test for the feature 'impl_trait_in_bindings'.
[00:04:40] Hint: create a failing test file named 'feature-gate-impl_trait_in_bindings.rs'
[00:04:40]       in the 'ui' test suite, with its failures due to
[00:04:40]       missing usage of #![feature(impl_trait_in_bindings)].
[00:04:40] Hint: If you already have such a test and don't want to rename it,
[00:04:40]       you can also add a // gate-test-impl_trait_in_bindings line to the test file.
[00:04:40] tidy error: Found 1 features without a gate test.
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:52
[00:04:41] Build completed unsuccessfully in 0:00:52
[00:04:41] Makefile:79: recipe for target 'tidy' failed
[00:04:41] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06001aee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0b821414:start=1536525943954970814,finish=1536525943963596718,duration=8625904
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0592ee64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ae064f
travis_time:start:04ae064f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:071e53d8
$ dmesg | grep -i kill
