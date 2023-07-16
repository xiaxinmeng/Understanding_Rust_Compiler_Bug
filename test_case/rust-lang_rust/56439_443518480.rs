plain
travis_time:end:09494bd3:start=1543765780359699405,finish=1543765781466632479,duration=1106933074
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:10] tidy error: /checkout/src/librustc/middle/liveness.rs:1661: line longer than 100 chars
[00:03:10] tidy error: /checkout/src/librustc/middle/liveness.rs:1664: line longer than 100 chars
[00:03:11] some tidy checks failed
[00:03:11] 
[00:03:11] 
[00:03:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:11] 
[00:03:11] 
[00:03:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:11] Build completed unsuccessfully in 0:00:58
[00:03:11] Build completed unsuccessfully in 0:00:58
[00:03:11] Makefile:79: recipe for target 'tidy' failed
[00:03:11] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:006cf5b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  2 15:53:01 UTC 2018
---
travis_time:end:02f303cf:start=1543765982192904323,finish=1543765982198075374,duration=5171051
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00bcb8b7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:37d76a8b
travis_time:start:37d76a8b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0031d646
$ dmesg | grep -i kill
