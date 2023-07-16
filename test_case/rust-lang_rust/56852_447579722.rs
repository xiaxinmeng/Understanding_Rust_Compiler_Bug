plain
travis_time:end:0356bc51:start=1544890514820219629,finish=1544890515860732604,duration=1040512975
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:02:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:42] tidy error: /checkout/src/ci/docker/x86_64-gnu-tools/checktools.sh:28: line longer than 100 chars
[00:02:44] some tidy checks failed
[00:02:44] 
[00:02:44] 
[00:02:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:44] 
[00:02:44] 
[00:02:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:44] Build completed unsuccessfully in 0:00:44
[00:02:44] Build completed unsuccessfully in 0:00:44
[00:02:44] make: *** [tidy] Error 1
[00:02:44] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:39837d28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 16:18:09 UTC 2018
---
travis_time:end:026ced64:start=1544890689468549565,finish=1544890689473895587,duration=5346022
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a15e6f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:32ddef1e
travis_time:start:32ddef1e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:094ef852
$ dmesg | grep -i kill
