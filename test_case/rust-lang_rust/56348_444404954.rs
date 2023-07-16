plain
travis_time:end:284da6b6:start=1543998742241291062,finish=1543998743276341899,duration=1035050837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:05] tidy error: /checkout/src/libcore/macros.rs:578: TODO is deprecated; use FIXME
[00:03:05] tidy error: /checkout/src/libcore/macros.rs:596: TODO is deprecated; use FIXME
[00:03:05] tidy error: /checkout/src/libcore/macros.rs:609: TODO is deprecated; use FIXME
[00:03:06] tidy error: /checkout/src/libstd/lib.rs:337: TODO is deprecated; use FIXME
[00:03:07] some tidy checks failed
[00:03:07] 
[00:03:07] 
[00:03:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:07] 
[00:03:07] 
[00:03:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:07] Build completed unsuccessfully in 0:00:56
[00:03:07] Build completed unsuccessfully in 0:00:56
[00:03:07] Makefile:79: recipe for target 'tidy' failed
[00:03:07] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18cd5548
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec  5 08:35:39 UTC 2018
---
travis_time:end:1aa5cc19:start=1543998939736380146,finish=1543998939740980375,duration=4600229
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05ae06dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a429d14
travis_time:start:0a429d14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c73da86
$ dmesg | grep -i kill
