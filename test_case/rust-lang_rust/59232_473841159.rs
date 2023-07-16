plain
travis_time:end:04007de0:start=1552901879025041602,finish=1552901881295070425,duration=2270028823
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:04] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1988: line longer than 100 chars
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:00:44
[00:04:06] Build completed unsuccessfully in 0:00:44
[00:04:06] Makefile:67: recipe for target 'tidy' failed
[00:04:06] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06f4286a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 18 09:42:17 UTC 2019
---
travis_time:end:03b038ce:start=1552902138318911738,finish=1552902138323506945,duration=4595207
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03d68f8e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19098a6e
travis_time:start:19098a6e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02096d33
$ dmesg | grep -i kill
