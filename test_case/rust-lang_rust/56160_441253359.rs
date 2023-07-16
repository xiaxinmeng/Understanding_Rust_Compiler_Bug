plain
travis_time:end:0b7af760:start=1542982218399679001,finish=1542982272372726294,duration=53973047293
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:48] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:49] tidy error: /checkout/src/test/ui/consts/const_let_eq.rs: missing trailing newline
[00:03:50] some tidy checks failed
[00:03:50] 
[00:03:50] 
[00:03:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:50] 
[00:03:50] 
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:50] Build completed unsuccessfully in 0:00:56
[00:03:50] Build completed unsuccessfully in 0:00:56
[00:03:50] make: *** [tidy] Error 1
[00:03:50] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09f01e60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 23 14:15:11 UTC 2018
---
travis_time:end:0a25ad5c:start=1542982511741445939,finish=1542982511746780386,duration=5334447
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08abd0a7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1db09bde
travis_time:start:1db09bde
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0498f810
$ dmesg | grep -i kill
