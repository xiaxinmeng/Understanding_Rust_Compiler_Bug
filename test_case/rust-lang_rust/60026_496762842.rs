plain
travis_time:end:02752a56:start=1559097167387587572,finish=1559097168209689654,duration=822102082
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/terminator.rs:10: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:56: trailing whitespace
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:574: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:578: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:664: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:684: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:697: trailing whitespace
[00:04:52] some tidy checks failed
[00:04:52] 
[00:04:52] 
[00:04:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:52] 
[00:04:52] 
[00:04:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:52] Build completed unsuccessfully in 0:01:12
[00:04:52] Build completed unsuccessfully in 0:01:12
[00:04:52] make: *** [tidy] Error 1
[00:04:52] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27789af2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 29 02:37:51 UTC 2019
---
travis_time:end:1f038874:start=1559097471905737640,finish=1559097471910094757,duration=4357117
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bfb89c9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08298745
travis_time:start:08298745
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c2d8d9f
$ dmesg | grep -i kill
