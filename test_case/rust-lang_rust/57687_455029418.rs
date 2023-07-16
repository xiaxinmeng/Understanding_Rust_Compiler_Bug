plain
travis_time:end:09bb9080:start=1547693414328576712,finish=1547693491038935389,duration=76710358677
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:29] tidy error: /checkout/src/libcore/pin.rs:132: trailing whitespace
[00:03:29] tidy error: /checkout/src/libcore/pin.rs:147: trailing whitespace
[00:03:31] some tidy checks failed
[00:03:31] 
[00:03:31] 
[00:03:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:31] 
[00:03:31] 
[00:03:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:31] Build completed unsuccessfully in 0:00:49
[00:03:31] Build completed unsuccessfully in 0:00:49
[00:03:31] make: *** [tidy] Error 1
[00:03:31] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00687d9e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 02:55:11 UTC 2019
---
travis_time:end:055765e0:start=1547693712282021902,finish=1547693712286971794,duration=4949892
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14efdbe6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:053c761a
travis_time:start:053c761a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c6925e6
$ dmesg | grep -i kill
