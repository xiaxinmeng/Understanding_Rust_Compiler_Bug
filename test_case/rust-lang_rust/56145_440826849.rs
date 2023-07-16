plain
travis_time:end:086f3864:start=1542837703580203705,finish=1542837757045396633,duration=53465192928
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
476 ./src/test/ui-fulldeps
472 ./src/libstd/sys/unix
468 ./src/test/ui/resolve
448 ./src/test/ui/traits
448 ./src/test/ui/re_rebalance_coherence
440 ./src/doc/unstable-book
436 ./src/test/ui/privacy
432 ./src/librustc_resolve
432 ./src/doc/unstable-book/src
---

[00:03:41] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:42] tidy error: /checkout/src/librustc/traits/coherence.rs:378: line longer than 100 chars
[00:03:43] some tidy checks failed
[00:03:43] 
[00:03:43] 
[00:03:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:43] 
[00:03:43] 
[00:03:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:43] Build completed unsuccessfully in 0:00:54
[00:03:43] Build completed unsuccessfully in 0:00:54
[00:03:43] Makefile:79: recipe for target 'tidy' failed
[00:03:43] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001d674e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 22:06:28 UTC 2018
---
travis_time:end:00464829:start=1542837989389297703,finish=1542837989393934802,duration=4637099
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16908eb3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08ca4cd1
travis_time:start:08ca4cd1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0150aba8
$ dmesg | grep -i kill
