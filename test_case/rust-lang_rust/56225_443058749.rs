plain
travis_time:end:02b7e4cd:start=1543541323834607666,finish=1543541326111413113,duration=2276805447
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:02:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:55] tidy error: /checkout/src/librustc_typeck/check/mod.rs:5025: line longer than 100 chars
[00:02:55] tidy error: /checkout/src/librustc_typeck/check/mod.rs:5270: line longer than 100 chars
[00:02:56] tidy error: Found 1 features without a gate test.
[00:02:56] Expected a gate test for the feature 'type_alias_enum_variants'.
[00:02:56] Hint: create a failing test file named 'feature-gate-type_alias_enum_variants.rs'
[00:02:56]       in the 'ui' test suite, with its failures due to
[00:02:56]       missing usage of #![feature(type_alias_enum_variants)].
[00:02:56] Hint: If you already have such a test and don't want to rename it,
[00:02:56]       you can also add a // gate-test-type_alias_enum_variants line to the test file.
[00:02:56] some tidy checks failed
[00:02:56] 
[00:02:56] 
[00:02:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:56] 
[00:02:56] 
[00:02:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:56] Build completed unsuccessfully in 0:00:54
[00:02:56] Build completed unsuccessfully in 0:00:54
[00:02:56] Makefile:79: recipe for target 'tidy' failed
[00:02:56] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02b5abb1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 01:31:51 UTC 2018
---
travis_time:end:095bfc1c:start=1543541512232132914,finish=1543541512236782853,duration=4649939
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:076594fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c13899c
travis_time:start:1c13899c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26a33126
$ dmesg | grep -i kill
