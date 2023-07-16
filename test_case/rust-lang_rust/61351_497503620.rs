plain
travis_time:end:1f1ee840:start=1559252560131854340,finish=1559252676774082914,duration=116642228574
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
[00:04:27] * 574 error codes
[00:04:27] * highest error code: E0729
[00:04:30] Expected a gate test for the feature 'doc_cfg'.
[00:04:30] Hint: create a failing test file named 'feature-gate-doc_cfg.rs'
[00:04:30]       in the 'ui' test suite, with its failures due to
[00:04:30]       missing usage of #![feature(doc_cfg)].
[00:04:30] Hint: If you already have such a test and don't want to rename it,
[00:04:30]       you can also add a // gate-test-doc_cfg line to the test file.
[00:04:30] tidy error: Found 1 features without a gate test.
[00:04:32] some tidy checks failed
[00:04:32] 
[00:04:32] 
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] 
[00:04:32] 
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:01:21
[00:04:32] Build completed unsuccessfully in 0:01:21
[00:04:32] Makefile:67: recipe for target 'tidy' failed
[00:04:32] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33c9ce70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 21:49:19 UTC 2019
---
travis_time:end:121190f7:start=1559252960000256035,finish=1559252960004647825,duration=4391790
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dd55b4e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13cb1c5a
travis_time:start:13cb1c5a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e3596f6
$ dmesg | grep -i kill
