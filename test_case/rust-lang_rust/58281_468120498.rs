plain
travis_time:end:03ce407e:start=1551322607919232831,finish=1551322680159608072,duration=72240375241
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:12: trailing whitespace
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:13: trailing whitespace
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:14: trailing whitespace
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:1229: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:679: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:63: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:84: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:130: line longer than 100 chars
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:256: line longer than 100 chars
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:299: TODO is deprecated; use FIXME
[00:03:46] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:347: TODO is deprecated; use FIXME
[00:03:48] some tidy checks failed
[00:03:48] 
[00:03:48] 
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:48] 
[00:03:48] 
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:00:50
[00:03:48] Build completed unsuccessfully in 0:00:50
[00:03:48] make: *** [tidy] Error 1
[00:03:48] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9ab7b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 03:01:58 UTC 2019
---
travis_time:end:08175020:start=1551322918891239143,finish=1551322918896311517,duration=5072374
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c86780e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:184eba06
travis_time:start:184eba06
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09160c43
$ dmesg | grep -i kill
