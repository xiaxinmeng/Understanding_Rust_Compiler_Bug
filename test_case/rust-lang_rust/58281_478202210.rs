plain
travis_time:end:09bf98b3:start=1553915612154711659,finish=1553915689371314962,duration=77216603303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:34] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:12: trailing whitespace
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:13: trailing whitespace
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14: trailing whitespace
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1229: TODO is deprecated; use FIXME
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:679: TODO is deprecated; use FIXME
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:63: TODO is deprecated; use FIXME
[00:03:35] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:66: TODO is deprecated; use FIXME
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:49
[00:03:37] Build completed unsuccessfully in 0:00:49
[00:03:37] Makefile:67: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3616660c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 30 03:18:36 UTC 2019
---
travis_time:end:0a80e604:start=1553915917265294577,finish=1553915917271114052,duration=5819475
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:086c0b80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02d8921c
travis_time:start:02d8921c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04238fa6
$ dmesg | grep -i kill
