plain
travis_time:end:0a77460b:start=1552957167327285107,finish=1552957244599926940,duration=77272641833
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:31] tidy error: /checkout/src/librustc/hir/lowering.rs:5139: line longer than 100 chars
[00:03:33] some tidy checks failed
[00:03:33] 
[00:03:33] 
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:33] 
[00:03:33] 
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] Build completed unsuccessfully in 0:00:47
[00:03:33] Build completed unsuccessfully in 0:00:47
[00:03:33] make: *** [tidy] Error 1
[00:03:33] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:012a6c4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 01:04:27 UTC 2019
---
travis_time:end:1ebb83b1:start=1552957468375444927,finish=1552957468380947856,duration=5502929
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017e29da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1018fe60
travis_time:start:1018fe60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0041fd44
$ dmesg | grep -i kill
