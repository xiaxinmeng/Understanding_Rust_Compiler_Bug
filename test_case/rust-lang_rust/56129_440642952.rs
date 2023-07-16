plain
travis_time:end:24003489:start=1542801778155365436,finish=1542801779250651935,duration=1095286499
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:56] 
##################################                                        48.2%
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating git repository `https://github.com/RalfJung/jobserver-rs`
[00:02:06]  Downloading crates ...
[00:02:06]   Downloaded serde_json v1.0.31
[00:02:06]   Downloaded toml v0.4.6
[00:02:06]   Downloaded cc v1.0.25
---
tidy check
[00:04:08] * 568 error codes
[00:04:08] * highest error code: E0721
[00:04:09] * 239 features
[00:04:09] invalid source: "git+https://github.com/RalfJung/jobserver-rs#3de40df347fcaeb0591dab77af47d3071ffa7b3b"
[00:04:09] some tidy checks failed
[00:04:09] 
[00:04:09] 
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:09] 
[00:04:09] 
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:09] Build completed unsuccessfully in 0:00:58
[00:04:09] Build completed unsuccessfully in 0:00:58
[00:04:09] make: *** [tidy] Error 1
[00:04:09] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03c89940
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 12:07:18 UTC 2018
---
travis_time:end:310d7828:start=1542802038447622999,finish=1542802038453731021,duration=6108022
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1352ccb1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23d0ab30
travis_time:start:23d0ab30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d6549be
$ dmesg | grep -i kill
