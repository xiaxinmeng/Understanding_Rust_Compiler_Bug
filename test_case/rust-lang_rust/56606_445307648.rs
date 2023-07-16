plain
travis_time:end:0218cb4b:start=1544203470799183111,finish=1544203471903294908,duration=1104111797
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
[00:03:34] * 568 error codes
[00:03:34] * highest error code: E0721
[00:03:34] * 242 features
[00:03:35] invalid source: "git+https://github.com/Zoxc/rust-smallvec?branch=push_light_def#b5c9040b44576c59580eccd085a53d4ddb78d587"
[00:03:35] some tidy checks failed
[00:03:35] 
[00:03:35] 
[00:03:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:35] 
[00:03:35] 
[00:03:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:35] Build completed unsuccessfully in 0:00:58
[00:03:35] Build completed unsuccessfully in 0:00:58
[00:03:35] Makefile:79: recipe for target 'tidy' failed
[00:03:35] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:084d5732
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 17:28:15 UTC 2018
---
travis_time:end:30883070:start=1544203696150186515,finish=1544203696155611994,duration=5425479
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b21d06
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10a895f0
travis_time:start:10a895f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:020070b2
$ dmesg | grep -i kill
