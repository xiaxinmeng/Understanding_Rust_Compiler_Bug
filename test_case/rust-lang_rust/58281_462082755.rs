plain
travis_time:end:03472770:start=1549748898701058846,finish=1549748899682723601,duration=981664755
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:12: trailing whitespace
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:13: trailing whitespace
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14: trailing whitespace
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:86: TODO is deprecated; use FIXME
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:160: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:286: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:329: TODO is deprecated; use FIXME
[00:04:02] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:377: TODO is deprecated; use FIXME
[00:04:03] some tidy checks failed
[00:04:03] 
[00:04:03] 
[00:04:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:03] 
[00:04:03] 
[00:04:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:03] Build completed unsuccessfully in 0:00:47
[00:04:03] Build completed unsuccessfully in 0:00:47
[00:04:03] Makefile:68: recipe for target 'tidy' failed
[00:04:03] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bb0ef77
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 21:52:34 UTC 2019
---
travis_time:end:184268d4:start=1549749155738072257,finish=1549749155743234535,duration=5162278
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0280b554
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:038459ca
travis_time:start:038459ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:262ff085
$ dmesg | grep -i kill
