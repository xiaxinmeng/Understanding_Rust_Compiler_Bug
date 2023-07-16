plain
travis_time:end:2435844f:start=1544653795282699541,finish=1544653935474823732,duration=140192124191
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:29] tidy error: /checkout/src/test/ui/consts/const-nonzero.rs: missing trailing newline
[00:03:30] some tidy checks failed
[00:03:30] 
[00:03:30] 
[00:03:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:30] 
[00:03:30] 
[00:03:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:30] Build completed unsuccessfully in 0:01:01
[00:03:30] Build completed unsuccessfully in 0:01:01
[00:03:30] Makefile:79: recipe for target 'tidy' failed
[00:03:30] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15dcf3b3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 12 22:35:55 UTC 2018
---
travis_time:end:04cfc58a:start=1544654155749368928,finish=1544654155754251930,duration=4883002
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00db2d32
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0168e8e0
travis_time:start:0168e8e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00573760
$ dmesg | grep -i kill
