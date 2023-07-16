plain
travis_time:end:17899ae0:start=1542472313833290308,finish=1542472315885864970,duration=2052574662
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:04:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:31] tidy error: /checkout/src/test/ui/consts/auxiliary/const_fn_lib.rs: missing trailing newline
[00:04:32] some tidy checks failed
[00:04:32] 
[00:04:32] 
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] 
[00:04:32] 
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:00:47
[00:04:32] Build completed unsuccessfully in 0:00:47
[00:04:32] make: *** [tidy] Error 1
[00:04:32] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c4f77d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Nov 17 16:36:38 UTC 2018
---
travis_time:end:0896f2be:start=1542472598986340535,finish=1542472598990933648,duration=4593113
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029a592e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19071ac0
travis_time:start:19071ac0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13dfaea9
$ dmesg | grep -i kill
