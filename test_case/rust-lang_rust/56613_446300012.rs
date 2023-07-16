plain
travis_time:end:0da1c2f0:start=1544550402261983730,finish=1544550404352765289,duration=2090781559
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:39] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:39] tidy error: /checkout/src/librustc/hir/map/mod.rs:164: trailing whitespace
[00:03:40] some tidy checks failed
[00:03:40] 
[00:03:40] 
[00:03:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:40] 
[00:03:40] 
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] Build completed unsuccessfully in 0:00:56
[00:03:40] Build completed unsuccessfully in 0:00:56
[00:03:40] Makefile:79: recipe for target 'tidy' failed
[00:03:40] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b3b7279
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 11 17:50:33 UTC 2018
---
travis_time:end:13d338e0:start=1544550634283219523,finish=1544550634289726110,duration=6506587
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c54bb64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1093ca94
travis_time:start:1093ca94
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:020a79c8
$ dmesg | grep -i kill
