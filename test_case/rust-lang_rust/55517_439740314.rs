plain
travis_time:end:0e3b66d9:start=1542586539491417189,finish=1542586595098754423,duration=55607337234
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:37] tidy error: /checkout/src/librustc/util/ppaux.rs:103: line longer than 100 chars
[00:03:37] tidy error: /checkout/src/librustc/util/ppaux.rs:1087: line longer than 100 chars
[00:03:38] some tidy checks failed
[00:03:38] 
[00:03:38] 
[00:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:38] 
[00:03:38] 
[00:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:38] Build completed unsuccessfully in 0:00:48
[00:03:38] Build completed unsuccessfully in 0:00:48
[00:03:38] Makefile:79: recipe for target 'tidy' failed
[00:03:38] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ecf9c26
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 00:20:23 UTC 2018
---
travis_time:end:1234d994:start=1542586823914938642,finish=1542586823920949783,duration=6011141
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd64e4b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07c6258a
travis_time:start:07c6258a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:051807d0
$ dmesg | grep -i kill
