plain
travis_time:end:03f329a0:start=1560326174685578468,finish=1560326264097964842,duration=89412386374
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:tidy
tidy check
[00:03:47] * 576 error codes
[00:03:47] * highest error code: E0731
[00:03:49] tidy error: Found 1 features without a gate test.
[00:03:49] Expected a gate test for the feature 'const_in_array_repeat_expressions'.
[00:03:49] Hint: create a failing test file named 'feature-gate-const_in_array_repeat_expressions.rs'
[00:03:49]       in the 'ui' test suite, with its failures due to
[00:03:49]       missing usage of #![feature(const_in_array_repeat_expressions)].
[00:03:49] Hint: If you already have such a test and don't want to rename it,
[00:03:49]       you can also add a // gate-test-const_in_array_repeat_expressions line to the test file.
[00:03:51] some tidy checks failed
[00:03:51] 
[00:03:51] 
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:51] 
[00:03:51] 
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:01:11
---
travis_time:end:20c5df26:start=1560326506144344571,finish=1560326506148810799,duration=4466228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09bc35c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07db5222
travis_time:start:07db5222
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11db4d18
$ dmesg | grep -i kill
