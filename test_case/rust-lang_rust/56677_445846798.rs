plain
travis_time:end:139af2a0:start=1544453473013273476,finish=1544453593381890974,duration=120368617498
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:45] tidy error: /checkout/src/test/compile-fail/must_use-in-stdlib-traits.rs: missing trailing newline
[00:03:46] some tidy checks failed
[00:03:46] 
[00:03:46] 
[00:03:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:46] 
[00:03:46] 
[00:03:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:46] Build completed unsuccessfully in 0:00:55
[00:03:46] Build completed unsuccessfully in 0:00:55
[00:03:46] make: *** [tidy] Error 1
[00:03:46] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ec004a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 10 14:57:09 UTC 2018
---
travis_time:end:01b68680:start=1544453829843660041,finish=1544453829848740622,duration=5080581
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:36217a40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a83ef09
travis_time:start:1a83ef09
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:013b87d6
$ dmesg | grep -i kill
