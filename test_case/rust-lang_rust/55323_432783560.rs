plain
travis_time:end:01ae2bf7:start=1540406309533525138,finish=1540406310565516447,duration=1031991309
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---

[00:03:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:52] tidy error: /checkout/src/librustc/dep_graph/dep_node.rs:75: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/librustc/ty/query/config.rs:17: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/librustc/ty/query/config.rs:119: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/librustc/ty/query/mod.rs:38: line longer than 100 chars
[00:03:54] some tidy checks failed
[00:03:54] 
[00:03:54] 
[00:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:54] 
[00:03:54] 
[00:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:54] Build completed unsuccessfully in 0:00:47
[00:03:54] Build completed unsuccessfully in 0:00:47
[00:03:54] make: *** [tidy] Error 1
[00:03:54] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:089c81f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cfce460:start=1540406556926874937,finish=1540406556931185348,duration=4310411
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13d2b9f1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02274316
travis_time:start:02274316
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:101c0f40
$ dmesg | grep -i kill
