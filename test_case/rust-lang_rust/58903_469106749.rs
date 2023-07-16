plain
travis_time:end:0949168d:start=1551669428319153791,finish=1551669501066998240,duration=72747844449
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:30] tidy error: /checkout/src/test/ui/parser/unclosed_delim_mod.rs: missing trailing newline
[00:03:31] some tidy checks failed
[00:03:31] 
[00:03:31] 
[00:03:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:31] 
[00:03:31] 
[00:03:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:31] Build completed unsuccessfully in 0:00:44
[00:03:31] Build completed unsuccessfully in 0:00:44
[00:03:31] Makefile:68: recipe for target 'tidy' failed
[00:03:31] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:062dae30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 03:22:01 UTC 2019
---
travis_time:end:07e903a4:start=1551669721858809196,finish=1551669721863614885,duration=4805689
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049553e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08014208
travis_time:start:08014208
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ca6aea
$ dmesg | grep -i kill
