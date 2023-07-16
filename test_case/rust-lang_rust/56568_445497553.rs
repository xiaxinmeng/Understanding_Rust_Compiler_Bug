plain
travis_time:end:002e2168:start=1544310664639759615,finish=1544310731686975554,duration=67047215939
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:47] tidy error: /checkout/src/libstd/sys/windows/args.rs:45: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/sys/windows/args.rs:224: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/sys/windows/args.rs:266: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/sys/windows/args.rs: missing trailing newline
[00:03:48] 
[00:03:48] 
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:48] 
[00:03:48] 
[00:03:48] some tidy checks failed
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:01:00
[00:03:48] make: *** [tidy] Error 1
[00:03:48] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:128bf5bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 23:16:10 UTC 2018
---
travis_time:end:0d0ecca0:start=1544310970629974320,finish=1544310970636492318,duration=6517998
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01424e73
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b3dc31
travis_time:start:03b3dc31
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04509466
$ dmesg | grep -i kill
