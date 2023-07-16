plain
travis_time:end:0ae6e514:start=1543177170638669348,finish=1543177224162988327,duration=53524318979
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:31] tidy error: /checkout/src/test/ui/deprecation/deprecation-in-future.rs:13: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustc/middle/stability.rs:567: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustc/middle/stability.rs:600: line longer than 100 chars
[00:03:33] some tidy checks failed
[00:03:33] 
[00:03:33] 
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:33] 
[00:03:33] 
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] Build completed unsuccessfully in 0:01:03
[00:03:33] Build completed unsuccessfully in 0:01:03
[00:03:33] make: *** [tidy] Error 1
[00:03:33] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05bb385c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 25 20:24:07 UTC 2018
---
travis_time:end:0ba0e3ee:start=1543177447430241162,finish=1543177447435223790,duration=4982628
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0807f89d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c22f1ce
travis_time:start:0c22f1ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04e2d13e
$ dmesg | grep -i kill
