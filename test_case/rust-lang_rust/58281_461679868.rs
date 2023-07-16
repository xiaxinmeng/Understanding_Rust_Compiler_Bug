plain
travis_time:end:0210b906:start=1549596446896856052,finish=1549596447927397751,duration=1030541699
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:4: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:4: TODO is deprecated; use FIXME
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:159: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:279: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:322: TODO is deprecated; use FIXME
[00:04:21] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/outlives_suggestion.rs:370: TODO is deprecated; use FIXME
[00:04:23] some tidy checks failed
[00:04:23] 
[00:04:23] 
[00:04:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:23] 
[00:04:23] 
[00:04:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:23] Build completed unsuccessfully in 0:00:47
[00:04:23] Build completed unsuccessfully in 0:00:47
[00:04:23] make: *** [tidy] Error 1
[00:04:23] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b8a1b33
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 03:32:02 UTC 2019
---
travis_time:end:02cfd210:start=1549596723818549187,finish=1549596723823693515,duration=5144328
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0196d8ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:169a1420
travis_time:start:169a1420
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b42cd5e
$ dmesg | grep -i kill
