plain
travis_time:end:016bd8f3:start=1544890870794757244,finish=1544890871809873525,duration=1015116281
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:17] tidy error: /checkout/src/ci/docker/x86_64-gnu-tools/checktools.sh:29: line longer than 100 chars
[00:03:19] some tidy checks failed
[00:03:19] 
[00:03:19] 
[00:03:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:19] 
[00:03:19] 
[00:03:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:19] Build completed unsuccessfully in 0:00:44
[00:03:19] Build completed unsuccessfully in 0:00:44
[00:03:19] Makefile:79: recipe for target 'tidy' failed
[00:03:19] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22649cb0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 16:24:39 UTC 2018
---
travis_time:end:02122f8a:start=1544891080368400872,finish=1544891080373932707,duration=5531835
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06b52e93
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25331de6
travis_time:start:25331de6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1919e860
$ dmesg | grep -i kill
