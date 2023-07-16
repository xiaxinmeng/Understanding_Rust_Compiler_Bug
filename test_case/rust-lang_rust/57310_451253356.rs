plain
travis_time:end:0209a86a:start=1546542387154158626,finish=1546542389779984410,duration=2625825784
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:02:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:52] tidy error: /checkout/src/librustc/diagnostics.rs:781: trailing whitespace
[00:02:53] some tidy checks failed
[00:02:53] 
[00:02:53] 
[00:02:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:53] 
[00:02:53] 
[00:02:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:53] Build completed unsuccessfully in 0:00:45
[00:02:53] Build completed unsuccessfully in 0:00:45
[00:02:53] make: *** [tidy] Error 1
[00:02:53] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:226af279
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 19:09:31 UTC 2019
---
travis_time:end:295838a4:start=1546542572317541423,finish=1546542572322685247,duration=5143824
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:186c5700
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15841fb0
travis_time:start:15841fb0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066f2f65
$ dmesg | grep -i kill
