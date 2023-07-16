plain
travis_time:end:0099f33c:start=1543513236546488962,finish=1543513238839969904,duration=2293480942
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:00] tidy error: /checkout/src/librustc_codegen_llvm/consts.rs:344: TODO is deprecated; use FIXME
[00:03:01] some tidy checks failed
[00:03:01] 
[00:03:01] 
[00:03:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:01] 
[00:03:01] 
[00:03:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:01] Build completed unsuccessfully in 0:00:54
[00:03:01] Build completed unsuccessfully in 0:00:54
[00:03:01] make: *** [tidy] Error 1
[00:03:01] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e8c59c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 29 17:43:49 UTC 2018
---
travis_time:end:18261aaa:start=1543513429885035375,finish=1543513429889595057,duration=4559682
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:064d16f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03cf10ef
travis_time:start:03cf10ef
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22737450
$ dmesg | grep -i kill
