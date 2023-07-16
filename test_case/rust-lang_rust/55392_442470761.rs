plain
travis_time:end:1770b8d8:start=1543415461496576628,finish=1543415464715278338,duration=3218701710
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
[00:03:05] tidy error: /checkout/src/librustc_mir/const_eval.rs:146: line longer than 100 chars
[00:03:07] some tidy checks failed
[00:03:07] 
[00:03:07] 
[00:03:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:07] 
[00:03:07] 
[00:03:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:07] Build completed unsuccessfully in 0:00:57
[00:03:07] Build completed unsuccessfully in 0:00:57
[00:03:07] Makefile:79: recipe for target 'tidy' failed
[00:03:07] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e1dcd00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 14:34:20 UTC 2018
---
travis_time:end:1f8ea9ac:start=1543415661250998543,finish=1543415661255737622,duration=4739079
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a16b1e7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09781cc7
travis_time:start:09781cc7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02d4383c
$ dmesg | grep -i kill
