plain
travis_time:end:2003a64c:start=1551181405958681542,finish=1551181408694382517,duration=2735700975
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:11:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:11:07] tidy error: /checkout/src/librustc/traits/object_safety.rs:562: line longer than 100 chars
[00:11:08] some tidy checks failed
[00:11:08] 
[00:11:08] 
[00:11:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:11:08] 
[00:11:08] 
[00:11:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:11:08] Build completed unsuccessfully in 0:00:46
[00:11:08] Build completed unsuccessfully in 0:00:46
[00:11:08] Makefile:68: recipe for target 'tidy' failed
[00:11:08] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:022355c9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 11:54:49 UTC 2019
---
travis_time:end:0560053b:start=1551182089952873668,finish=1551182089957490399,duration=4616731
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e88acec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09a436d1
travis_time:start:09a436d1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0afbb6dd
$ dmesg | grep -i kill
