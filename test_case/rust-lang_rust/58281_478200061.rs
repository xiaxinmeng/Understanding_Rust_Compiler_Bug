plain
travis_time:end:25f08fce:start=1553913728968451495,finish=1553913807952534945,duration=78984083450
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:12: trailing whitespace
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:13: trailing whitespace
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14: trailing whitespace
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1229: TODO is deprecated; use FIXME
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:679: TODO is deprecated; use FIXME
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:63: TODO is deprecated; use FIXME
[00:03:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:66: TODO is deprecated; use FIXME
[00:03:23] some tidy checks failed
[00:03:23] 
[00:03:23] 
[00:03:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:23] 
[00:03:23] 
[00:03:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:23] Build completed unsuccessfully in 0:00:45
[00:03:23] Build completed unsuccessfully in 0:00:45
[00:03:23] make: *** [tidy] Error 1
[00:03:23] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:145385df
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 30 02:47:00 UTC 2019
---
travis_time:end:11edd6f5:start=1553914021509939662,finish=1553914021514966774,duration=5027112
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:077a26f3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10573590
travis_time:start:10573590
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0984c282
$ dmesg | grep -i kill
