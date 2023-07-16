plain
travis_time:end:0066eee4:start=1547692080936907462,finish=1547692082378653094,duration=1441745632
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:32] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:32] tidy error: /checkout/src/libcore/pin.rs:132: trailing whitespace
[00:03:32] tidy error: /checkout/src/libcore/pin.rs:147: trailing whitespace
[00:03:34] some tidy checks failed
[00:03:34] 
[00:03:34] 
[00:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:34] 
[00:03:34] 
[00:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:34] Build completed unsuccessfully in 0:00:47
[00:03:34] Build completed unsuccessfully in 0:00:47
[00:03:34] Makefile:69: recipe for target 'tidy' failed
[00:03:34] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:094e3674
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 02:31:48 UTC 2019
---
travis_time:end:2155442a:start=1547692308763340948,finish=1547692308767892582,duration=4551634
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021f97ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036b24ff
travis_time:start:036b24ff
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1012a880
$ dmesg | grep -i kill
