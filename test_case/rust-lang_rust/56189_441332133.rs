plain
travis_time:end:021b8011:start=1543017798489266461,finish=1543017859998675257,duration=61509408796
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:16] tidy error: /checkout/src/librustdoc/test.rs:271: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/librustdoc/test.rs:278: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/librustdoc/test.rs:309: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/librustdoc/test.rs:389: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/librustdoc/test.rs:622: line longer than 100 chars
[00:03:17] some tidy checks failed
[00:03:17] 
[00:03:17] 
[00:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:17] 
[00:03:17] 
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:17] Build completed unsuccessfully in 0:00:57
[00:03:17] Build completed unsuccessfully in 0:00:57
[00:03:17] Makefile:79: recipe for target 'tidy' failed
[00:03:17] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0097cf20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Nov 24 00:07:46 UTC 2018
---
travis_time:end:03998c14:start=1543018067118755215,finish=1543018067124316553,duration=5561338
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dbec2e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02c2b270
travis_time:start:02c2b270
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03fa6259
$ dmesg | grep -i kill
