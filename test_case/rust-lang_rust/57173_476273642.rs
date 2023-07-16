plain
travis_time:end:03e34fb5:start=1553529855727702413,finish=1553529858269195928,duration=2541493515
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:03] tidy error: /checkout/src/librustc/hir/lowering.rs:2745: line longer than 100 chars
[00:04:04] some tidy checks failed
[00:04:04] 
[00:04:04] 
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:04] 
[00:04:04] 
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:46
[00:04:04] Build completed unsuccessfully in 0:00:46
[00:04:04] Makefile:67: recipe for target 'tidy' failed
[00:04:04] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12f7872c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 16:08:33 UTC 2019
---
travis_time:end:0b00b458:start=1553530114626071523,finish=1553530114631118996,duration=5047473
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14449820
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1451825d
travis_time:start:1451825d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:047514d6
$ dmesg | grep -i kill
