plain
travis_time:end:0ed17820:start=1543205886301110705,finish=1543205939631521956,duration=53330411251
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:tidy
tidy check
[00:03:39] * 568 error codes
[00:03:39] * highest error code: E0721
[00:03:40] Expected a gate test for the feature 'type_alias_enum_variants'.
[00:03:40] Hint: create a failing test file named 'feature-gate-type_alias_enum_variants.rs'
[00:03:40]       in the 'ui' test suite, with its failures due to
[00:03:40]       missing usage of #![feature(type_alias_enum_variants)].
[00:03:40] Hint: If you already have such a test and don't want to rename it,
[00:03:40]       you can also add a // gate-test-type_alias_enum_variants line to the test file.
[00:03:40] tidy error: Found 1 features without a gate test.
[00:03:40] some tidy checks failed
[00:03:40] 
[00:03:40] 
[00:03:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:40] 
[00:03:40] 
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] Build completed unsuccessfully in 0:00:55
[00:03:40] Build completed unsuccessfully in 0:00:55
[00:03:40] Makefile:79: recipe for target 'tidy' failed
[00:03:40] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dd58282
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 04:22:48 UTC 2018
---
travis_time:end:1d587f9c:start=1543206169243430844,finish=1543206169249593907,duration=6163063
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b37b25e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1af3b320
travis_time:start:1af3b320
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0993e4de
$ dmesg | grep -i kill
