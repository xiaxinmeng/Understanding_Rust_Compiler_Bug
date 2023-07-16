plain
travis_time:end:0438165e:start=1540810153829036941,finish=1540810154845179726,duration=1016142785
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---

[00:04:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:211: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:384: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:474: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:584: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:711: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:739: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:740: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:757: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:779: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:816: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:829: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:859: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:862: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:967: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1001: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1103: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1112: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1253: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1256: line longer than 100 chars
[00:04:24] tidy error: /checkout/src/test/run-pass/issues/issue-54477-reduced-1.rs:1272: line longer than 100 chars
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:00:46
[00:04:26] Build completed unsuccessfully in 0:00:46
[00:04:26] Makefile:79: recipe for target 'tidy' failed
[00:04:26] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0096d290
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0340d254:start=1540810431490937281,finish=1540810431496515213,duration=5577932
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:354ebbae
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06f0c142
travis_time:start:06f0c142
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0761e0d8
$ dmesg | grep -i kill
