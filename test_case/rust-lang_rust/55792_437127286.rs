plain
travis_time:end:00ff383b:start=1541704803753580481,finish=1541704806858088500,duration=3104508019
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:44] tidy error: /checkout/src/test/ui/consts/const-prop-ice2.rs: missing trailing newline
[00:03:44] tidy error: /checkout/src/test/ui/consts/const-prop-ice.rs: missing trailing newline
[00:03:45] some tidy checks failed
[00:03:45] 
[00:03:45] 
[00:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:45] 
[00:03:45] 
[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] Build completed unsuccessfully in 0:00:47
[00:03:45] Build completed unsuccessfully in 0:00:47
[00:03:45] Makefile:79: recipe for target 'tidy' failed
[00:03:45] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d9b63be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov  8 19:24:03 UTC 2018
---
travis_time:end:0832ae23:start=1541705043549877479,finish=1541705043556118771,duration=6241292
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cdc734e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0054cac8
travis_time:start:0054cac8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a437dac
$ dmesg | grep -i kill
