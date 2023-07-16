plain
travis_time:end:0384c2ef:start=1543315456431473954,finish=1543315528561872868,duration=72130398914
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
tidy check
[00:03:11] * 568 error codes
[00:03:11] * highest error code: E0721
[00:03:11] * 239 features
[00:03:12] Stray file with UI testing output: "/checkout/src/test/ui/regions/regions-struct-not-wf.stderr"
[00:03:12] some tidy checks failed
[00:03:12] 
[00:03:12] 
[00:03:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:12] 
[00:03:12] 
[00:03:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:12] Build completed unsuccessfully in 0:00:57
[00:03:12] Build completed unsuccessfully in 0:00:57
[00:03:12] Makefile:79: recipe for target 'tidy' failed
[00:03:12] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11a2f20e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 10:48:49 UTC 2018
---
travis_time:end:26893848:start=1543315730308078995,finish=1543315730312624887,duration=4545892
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19cd3df1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03113ed8
travis_time:start:03113ed8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0240777c
$ dmesg | grep -i kill
